#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{debug, traits::Randomness, traits::{Currency, LockableCurrency}};
use frame_system::offchain::SubmitTransaction;
use lite_json::json::JsonValue;
use sp_core::H256;
use sp_runtime::{offchain::{http, Duration}, traits::{SaturatedConversion, CheckedDiv, CheckedMul}};
use sp_std::{str, vec::Vec};
use dbc_price_trait::DbcPrice;

pub use pallet::*;

type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::{offchain::CreateSignedTransaction, pallet_prelude::*};
    use sp_std::vec::Vec;

    /// The type to sign and send transactions.
    pub const UNSIGNED_TXS_PRIORITY: u64 = 100;
    pub const MAX_LEN: usize = 64;
    type URL = Vec<u8>;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + CreateSignedTransaction<Call<Self>> + generic_func::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type RandomnessSource: Randomness<H256>;
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn prices)]
    pub type Prices<T> = StorageValue<_, Vec<u64>, ValueQuery>;

    // https://min-api.cryptocompare.com/data/price?fsym=DBC&tsyms=USD
    #[pallet::storage]
    #[pallet::getter(fn price_url)]
    pub(super) type PriceURL<T> = StorageValue<_, Vec<URL>>;

    /// avgPrice = price * 10**6 usd
    #[pallet::storage]
    #[pallet::getter(fn avg_price)]
    pub(super) type AvgPrice<T> = StorageValue<_, u64>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AddNewPrice(u64),
        AddAvgPrice(u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoLocalAcctForSigning,
        FetchPriceFailed,
        OffchainUnsignedTxSignedPayloadError,
        OffchainUnsignedTxError,
        NoneValue,
        IndexOutOfRange,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            if let None = Self::price_url() {
                return
            }

            debug::native::info!(
                "Hello world from offchain worker at height: {:#?}!",
                block_number
            );

            let result = Self::fetch_price_and_send_unsigned_tx();
            if let Err(e) = result {
                debug::error!("offchain_worker error: {:?}", e);
                return;
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn submit_price_unsigned(origin: OriginFor<T>, price: u64) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;

            Self::add_price(price);
            Self::add_avg_price();

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn add_price_url(origin: OriginFor<T>, new_url: URL) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if let Some(mut price_url) = Self::price_url() {
                price_url.push(new_url);
                PriceURL::<T>::put(price_url);
            } else {
                let mut out = Vec::new();
                out.push(new_url);
                PriceURL::<T>::put(out);
            }

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn rm_price_url_by_index(origin: OriginFor<T>, index: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if let Some(mut price_url) = Self::price_url() {
                ensure!(index < price_url.len() as u32, Error::<T>::IndexOutOfRange);
                price_url.remove(index as usize);
                PriceURL::<T>::put(price_url);
            } else {
                return Err(Error::<T>::IndexOutOfRange.into())
            }

            Ok(().into())
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            let valid_tx = |provide| {
                ValidTransaction::with_tag_prefix("dbc-price")
                    .priority(UNSIGNED_TXS_PRIORITY)
                    .and_provides([&provide])
                    .longevity(3)
                    .propagate(true)
                    .build()
            };

            match call {
                Call::submit_price_unsigned(_price) => valid_tx(b"submit_price_unsigned".to_vec()),
                _ => InvalidTransaction::Call.into(),
            }
        }
    }
}

impl<T: Config> Pallet<T> {
    fn gen_rand_url() -> Option<u32> {
        let price_url = Self::price_url()?;
        return Some(<generic_func::Module<T>>::random_u32((price_url.len() - 1) as u32));
    }

    fn fetch_price_and_send_unsigned_tx() -> Result<(), Error<T>> {
        let price = Self::fetch_price().map_err(|_| <Error<T>>::FetchPriceFailed)?;

        let call = Call::submit_price_unsigned(price);
        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()).map_err(|_| {
            debug::error!("Failed in offchain_unsigned_tx"); // TODO: error here
            <Error<T>>::OffchainUnsignedTxError
        })
    }

    // 获取并返回当前价格
    fn fetch_price() -> Result<u64, http::Error> {
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(4_000));

        let price_url = Self::price_url().ok_or(http::Error::Unknown)?;

        let rand_price_url_index = Self::gen_rand_url().ok_or(http::Error::Unknown)?;

        let price_url = str::from_utf8(&price_url[rand_price_url_index as usize])
            .map_err(|_| http::Error::Unknown)?;

        let request = http::Request::get(price_url);

        let pending = request
            .deadline(deadline)
            .send()
            .map_err(|_| http::Error::IoError)?;

        let response = pending
            .try_wait(deadline)
            .map_err(|_| http::Error::DeadlineReached)??;
        // Let's check the status code before we proceed to reading the response.
        if response.code != 200 {
            debug::warn!("Unexpected status code: {}", response.code);
            return Err(http::Error::Unknown);
        }
        let body = response.body().collect::<Vec<u8>>();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            debug::warn!("No UTF8 body");
            http::Error::Unknown
        })?;

        let price = match Self::parse_price(body_str) {
            Some(price) => Ok(price),
            None => {
                debug::warn!("Unable to extract price from the response: {:?}", body_str);
                Err(http::Error::Unknown)
            }
        }?;

        debug::warn!("Got price: {} cents", price);

        Ok(price)
    }

    // 将str价格转为u64 (*10^6)
    fn parse_price(price_str: &str) -> Option<u64> {
        let val = lite_json::parse_json(price_str);
        let price = val.ok().and_then(|v| match v {
            JsonValue::Object(obj) => {
                let mut chars = "USD".chars();
                obj.into_iter()
                    .find(|(k, _)| k.iter().all(|k| Some(*k) == chars.next()))
                    .and_then(|v| match v.1 {
                        JsonValue::Number(number) => Some(number),
                        _ => None,
                    })
            }
            _ => None,
        })?;

        // out = price.integer * 10**6 + price.fraction / 10**fraction_length * 10**6
        let fraction = price.fraction * 10_u64.pow(6) / 10_u64.pow(price.fraction_length);
        Some(price.integer as u64 * 1000_000 + fraction)
    }

    // 存储获取到的价格
    fn add_price(price: u64) {
        debug::info!("Adding to the average: {}", price);
        let mut prices = Prices::<T>::get();
        if prices.len() < MAX_LEN {
            prices.push(price);
        } else {
            prices[price as usize % MAX_LEN] = price;
        }

        Prices::<T>::put(prices);

        Self::deposit_event(Event::AddNewPrice(price));
    }

    // TODO: 可以增加去除最低分，最高分
    fn add_avg_price() {
        let prices = Prices::<T>::get();
        if prices.len() != MAX_LEN {
            return;
        }
        let avg_price =
            prices.iter().fold(0_u64, |a, b| a.saturating_add(*b)) / prices.len() as u64;

        debug::info!("Current average price: {:?}", avg_price);

        AvgPrice::<T>::put(avg_price);
        Self::deposit_event(Event::AddAvgPrice(avg_price));
    }
}

impl<T: Config> DbcPrice for Pallet<T> {
    type Balance = BalanceOf<T>;

    fn get_dbc_amount_by_value(value: u64) -> Option<Self::Balance> {
        let one_dbc: Self::Balance = 1000_000_000_000_000u64.saturated_into();
        let dbc_price: Self::Balance = Self::avg_price()?.saturated_into();
        value.saturated_into::<Self::Balance>().checked_mul(&one_dbc)?.checked_div(&dbc_price)
    }
}