#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use sp_runtime::traits::MaybeDisplay;
use sp_std::prelude::Vec;

use dbc_support::{
    rental_type::{MachineGPUOrder, RentOrderDetail},
    MachineId, RentOrderId,
};

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait RmRpcApi<AccountId, BlockNumber, Balance> where
        AccountId: codec::Codec + Ord,
        BlockNumber: Codec + MaybeDisplay,
        Balance: Codec + MaybeDisplay,
    {
        fn get_rent_order(rent_id: RentOrderId) -> RentOrderDetail<AccountId, BlockNumber, Balance>;
        fn get_rent_list(renter: AccountId) -> Vec<RentOrderId>;

        fn is_machine_renter(machine_id: MachineId, renter: AccountId) -> bool;
        fn get_machine_rent_id(machine_id: MachineId) -> MachineGPUOrder;
    }
}
