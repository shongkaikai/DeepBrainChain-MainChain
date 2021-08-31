#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    ensure,
    pallet_prelude::*,
    traits::{Currency, LockableCurrency},
};
use frame_system::{ensure_signed, pallet_prelude::*};
use online_profile::CommitteeUploadInfo;
use online_profile_machine::{ManageCommittee, OCOps};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::{prelude::*, str, vec::Vec};

mod rpc_types;
pub use rpc_types::RpcOCCommitteeOps;

pub type MachineId = Vec<u8>;
pub type EraIndex = u32;
type BalanceOf<T> = <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// 36 hours divide into 9 intervals for verification
pub const DISTRIBUTION: u32 = 9;
/// Each committee have 480 blocks (4 hours) to verify machine
pub const DURATIONPERCOMMITTEE: u32 = 480;
/// After order distribution 36 hours, allow committee submit raw info
pub const SUBMIT_RAW_START: u32 = 4320;
/// Summary committee's opinion after 48 hours
pub const SUBMIT_RAW_END: u32 = 5760;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Query distributed machines by committee address
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct OCCommitteeMachineList {
    /// machines, that distributed to committee, and should be verified
    pub booked_machine: Vec<MachineId>,
    /// machines, have submited machine info hash
    pub hashed_machine: Vec<MachineId>,
    /// machines, have submited raw machine info
    pub confirmed_machine: Vec<MachineId>,
    /// machines, online successfully
    pub online_machine: Vec<MachineId>,
}

impl OCCommitteeMachineList {
    fn rm_one(a_field: &mut Vec<MachineId>, machine_id: &MachineId) {
        if let Ok(index) = a_field.binary_search(machine_id) {
            a_field.remove(index);
        }
    }
}

/// Machines' verifying committee
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct OCMachineCommitteeList<AccountId, BlockNumber> {
    /// When order distribution happened
    pub book_time: BlockNumber,
    /// Committees, get the job to verify machine info
    pub booked_committee: Vec<AccountId>,
    /// Committees, have submited machine info hash
    pub hashed_committee: Vec<AccountId>,
    /// When committee can submit raw machine info, submit machine info can
    /// immediately start after all booked_committee submit hash
    pub confirm_start_time: BlockNumber,
    /// Committees, have submit raw machine info
    pub confirmed_committee: Vec<AccountId>,
    /// Committees, get a consensus, so can get rewards after machine online
    pub onlined_committee: Vec<AccountId>,
    /// Current order status
    pub status: OCVerifyStatus,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum OCVerifyStatus {
    SubmittingHash,
    SubmittingRaw,
    Summarizing,
    Finished,
}

impl Default for OCVerifyStatus {
    fn default() -> Self {
        OCVerifyStatus::SubmittingHash
    }
}

/// A record of committee’s operations when verifying machine info
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct OCCommitteeOps<BlockNumber, Balance> {
    pub staked_dbc: Balance,
    /// When one committee can start the virtual machine to verify machine info
    pub verify_time: Vec<BlockNumber>,
    pub confirm_hash: [u8; 16],
    pub hash_time: BlockNumber,
    /// When one committee submit raw machine info
    pub confirm_time: BlockNumber,
    pub machine_status: OCMachineStatus,
    pub machine_info: CommitteeUploadInfo,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum OCMachineStatus {
    Booked,
    Hashed,
    Confirmed,
}

impl Default for OCMachineStatus {
    fn default() -> Self {
        OCMachineStatus::Booked
    }
}

/// What will happen after all committee submit raw machine info
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum MachineConfirmStatus<AccountId> {
    /// Machine is confirmed by committee, so can be online later
    Confirmed(Summary<AccountId>),
    /// Machine is refused, will not online
    Refuse(Summary<AccountId>),
    /// No consensus, so machine will be redistributed and verified later
    NoConsensus(Summary<AccountId>),
}

impl<AccountId: Default> Default for MachineConfirmStatus<AccountId> {
    fn default() -> Self {
        Self::Confirmed(Summary { ..Default::default() })
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Summary<AccountId> {
    /// Machine will be online, and those committee will get reward
    pub valid_support: Vec<AccountId>,
    /// Machine will be online, and those committee cannot get reward
    /// for they submit different message from majority committee
    pub invalid_support: Vec<AccountId>,
    /// Committees, that not submit all message
    /// such as: not submit hash, not submit raw info before deadline
    pub unruly: Vec<AccountId>,
    /// Committees, refuse machine online
    pub against: Vec<AccountId>,
    /// Raw machine info, most majority committee submit
    pub info: Option<CommitteeUploadInfo>,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + online_profile::Config + generic_func::Config + committee::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
        type OCOperations: OCOps<
            AccountId = Self::AccountId,
            MachineId = MachineId,
            CommitteeUploadInfo = CommitteeUploadInfo,
        >;
        type ManageCommittee: ManageCommittee<AccountId = Self::AccountId, BalanceOf = BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_block_number: T::BlockNumber) {
            Self::statistic_result();
            Self::distribute_machines();
        }
    }

    // 存储用户订阅的不同确认阶段的机器
    #[pallet::storage]
    #[pallet::getter(fn committee_machine)]
    pub(super) type CommitteeMachine<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, OCCommitteeMachineList, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn machine_committee)]
    pub(super) type MachineCommittee<T: Config> =
        StorageMap<_, Blake2_128Concat, MachineId, OCMachineCommitteeList<T::AccountId, T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn machine_submited_hash)]
    pub(super) type MachineSubmitedHash<T> = StorageMap<_, Blake2_128Concat, MachineId, Vec<[u8; 16]>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn committee_ops)]
    pub(super) type CommitteeOps<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        MachineId,
        OCCommitteeOps<T::BlockNumber, BalanceOf<T>>,
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // 添加确认hash
        #[pallet::weight(10000)]
        pub fn submit_confirm_hash(
            origin: OriginFor<T>,
            machine_id: MachineId,
            hash: [u8; 16],
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let now = <frame_system::Module<T>>::block_number();

            let mut machine_committee = Self::machine_committee(&machine_id);
            let mut machine_submited_hash = Self::machine_submited_hash(&machine_id);

            // 从机器信息列表中有该委员会
            machine_committee.booked_committee.binary_search(&who).map_err(|_| Error::<T>::NotInBookList)?;

            // 该委员会没有提交过Hash
            if machine_committee.hashed_committee.binary_search(&who).is_ok() {
                return Err(Error::<T>::AlreadySubmitHash.into())
            }

            // 检查该Hash应该未出现过
            if let Err(index) = machine_submited_hash.binary_search(&hash) {
                machine_submited_hash.insert(index, hash.clone());
            } else {
                return Err(Error::<T>::DuplicateHash.into())
            }

            // 在该机器信息中，记录上委员的Hash
            if let Err(index) = machine_committee.hashed_committee.binary_search(&who) {
                machine_committee.hashed_committee.insert(index, who.clone());
            }

            let mut committee_machine = Self::committee_machine(&who);

            // 从委员的任务中，删除该机器的任务
            if let Ok(index) = committee_machine.booked_machine.binary_search(&machine_id) {
                committee_machine.booked_machine.remove(index);
            }
            // 委员会hashedmachine添加上该机器
            if let Err(index) = committee_machine.hashed_machine.binary_search(&machine_id) {
                committee_machine.hashed_machine.insert(index, machine_id.clone());
            }

            // 添加用户对机器的操作记录
            let mut committee_ops = Self::committee_ops(&who, &machine_id);
            committee_ops.machine_status = OCMachineStatus::Hashed;
            committee_ops.confirm_hash = hash.clone();
            committee_ops.hash_time = now;

            // 如果委员会都提交了Hash,则直接进入提交原始信息的阶段
            if machine_committee.booked_committee.len() == machine_committee.hashed_committee.len() {
                machine_committee.status = OCVerifyStatus::SubmittingRaw;
            }

            // 更新存储
            MachineSubmitedHash::<T>::insert(&machine_id, machine_submited_hash);
            MachineCommittee::<T>::insert(&machine_id, machine_committee);
            CommitteeMachine::<T>::insert(&who, committee_machine);
            CommitteeOps::<T>::insert(&who, &machine_id, committee_ops);

            Self::deposit_event(Event::AddConfirmHash(who, hash));

            Ok(().into())
        }

        /// 委员会提交的原始信息
        #[pallet::weight(10000)]
        pub fn submit_confirm_raw(
            origin: OriginFor<T>,
            machine_info_detail: CommitteeUploadInfo,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let now = <frame_system::Module<T>>::block_number();

            let machine_id = machine_info_detail.machine_id.clone();

            let mut machine_committee = Self::machine_committee(&machine_id);
            let mut committee_machine = Self::committee_machine(&who);
            let mut machine_ops = Self::committee_ops(&who, &machine_id);

            // 如果所有人都提交了，则直接可以提交Hash
            if machine_committee.status != OCVerifyStatus::SubmittingRaw {
                // 查询是否已经到了提交hash的时间 必须在36 ~ 48小时之间
                ensure!(now >= machine_committee.confirm_start_time, Error::<T>::TimeNotAllow);
                ensure!(now <= machine_committee.book_time + SUBMIT_RAW_END.into(), Error::<T>::TimeNotAllow);
            }

            // 该用户已经给机器提交过Hash
            machine_committee.hashed_committee.binary_search(&who).map_err(|_| Error::<T>::NotSubmitHash)?;

            // 机器ID存在于用户已经Hash的机器里
            committee_machine.hashed_machine.binary_search(&machine_id).map_err(|_| Error::<T>::NotSubmitHash)?;

            // 检查提交的raw与已提交的Hash一致
            let info_hash = machine_info_detail.hash();
            ensure!(info_hash == machine_ops.confirm_hash, Error::<T>::InfoNotFeatHash);

            // 用户还未提交过原始信息
            if committee_machine.confirmed_machine.binary_search(&machine_id).is_ok() {
                return Err(Error::<T>::AlreadySubmitRaw.into())
            }

            // 修改存储
            if let Ok(index) = committee_machine.hashed_machine.binary_search(&machine_id) {
                committee_machine.hashed_machine.remove(index);
            }
            if let Err(index) = committee_machine.confirmed_machine.binary_search(&machine_id) {
                committee_machine.confirmed_machine.insert(index, machine_id.clone());
            }

            if let Err(index) = machine_committee.confirmed_committee.binary_search(&who) {
                machine_committee.confirmed_committee.insert(index, who.clone());
            }

            // machine_ops.confirm_raw = confirm_raw.clone();
            machine_ops.confirm_time = now;
            machine_ops.machine_status = OCMachineStatus::Confirmed;
            machine_ops.machine_info = machine_info_detail.clone();
            machine_ops.machine_info.rand_str = Vec::new();

            // 如果全部都提交完了原始信息，则允许进入summary
            if machine_committee.confirmed_committee.len() == machine_committee.hashed_committee.len() {
                machine_committee.status = OCVerifyStatus::Summarizing;
            }

            CommitteeMachine::<T>::insert(&who, committee_machine);
            MachineCommittee::<T>::insert(&machine_id, machine_committee);
            CommitteeOps::<T>::insert(&who, &machine_id, machine_ops);

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AddConfirmHash(T::AccountId, [u8; 16]),
    }

    #[pallet::error]
    pub enum Error<T> {
        NotInBookList,
        AlreadySubmitHash,
        NotAllHashSubmited,
        TimeNotAllow,
        NotSubmitHash,
        AlreadySubmitRaw,
        InfoNotFeatHash,
        DuplicateHash,
    }
}

impl<T: Config> Pallet<T> {
    // 获取所有新加入的机器，并进行分派给委员会
    pub fn distribute_machines() {
        let live_machines = <online_profile::Pallet<T>>::live_machines();
        for a_machine_id in live_machines.confirmed_machine {
            // 重新分配: 必须清空该状态
            if MachineCommittee::<T>::contains_key(&a_machine_id) {
                MachineCommittee::<T>::remove(&a_machine_id);
            }

            let _ = Self::distribute_one_machine(&a_machine_id);
        }
    }

    pub fn distribute_one_machine(machine_id: &MachineId) -> Result<(), ()> {
        let lucky_committee = Self::lucky_committee().ok_or(())?;

        // 每个添加4个小时
        let now = <frame_system::Module<T>>::block_number();
        let confirm_start = now + SUBMIT_RAW_START.into(); // 添加确认信息时间为分发之后的36小时

        for a_book in lucky_committee {
            let _ = Self::book_one(machine_id.to_vec(), confirm_start, now, a_book);
        }

        // 将机器状态从ocw_confirmed_machine改为booked_machine
        T::OCOperations::oc_booked_machine(machine_id.clone());
        Ok(())
    }

    // 一个委员会进行操作
    // - Writes: MachineCommittee, CommitteeMachine, CommitteeOps
    fn book_one(
        machine_id: MachineId,
        confirm_start: T::BlockNumber,
        now: T::BlockNumber,
        order_time: (T::AccountId, Vec<usize>),
    ) -> Result<(), ()> {
        // 增加质押：由committee执行
        let stake_need = <T as pallet::Config>::ManageCommittee::stake_per_order().ok_or(())?;
        <T as pallet::Config>::ManageCommittee::change_used_stake(&order_time.0, stake_need, true)?;

        // 修改machine对应的委员会
        let mut machine_committee = Self::machine_committee(&machine_id);
        machine_committee.book_time = now;

        if let Err(index) = machine_committee.booked_committee.binary_search(&order_time.0) {
            machine_committee.booked_committee.insert(index, order_time.0.clone());
        }
        machine_committee.confirm_start_time = confirm_start;

        // 修改委员会对应的machine
        let mut committee_machine = Self::committee_machine(&order_time.0);
        if let Err(index) = committee_machine.booked_machine.binary_search(&machine_id) {
            committee_machine.booked_machine.insert(index, machine_id.clone());
        }

        // 修改委员会的操作
        let mut committee_ops = OCCommitteeOps { ..Default::default() };
        committee_ops.staked_dbc = stake_need;
        let start_time: Vec<_> = order_time
            .1
            .into_iter()
            .map(|x| now + (x as u32 * SUBMIT_RAW_START / DISTRIBUTION).into())
            .collect();
        committee_ops.verify_time = start_time;
        committee_ops.machine_status = OCMachineStatus::Booked;

        // 存储变量
        MachineCommittee::<T>::insert(&machine_id, machine_committee);
        CommitteeMachine::<T>::insert(&order_time.0, committee_machine);
        CommitteeOps::<T>::insert(&order_time.0, &machine_id, committee_ops);

        Ok(())
    }

    // 分派一个machineId给随机的委员会
    // 返回Distribution(9)个随机顺序的账户列表
    pub fn lucky_committee() -> Option<Vec<(T::AccountId, Vec<usize>)>> {
        let mut committee = <committee::Module<T>>::available_committee().ok()?;
        // 选出lucky_committee_num个委员会
        let mut lucky_committee = Vec::new();

        // Require committee_num at lease 3
        let lucky_committee_num = if committee.len() < 3 { return None } else { 3 };

        for _ in 0..lucky_committee_num {
            let lucky_index = <generic_func::Module<T>>::random_u32(committee.len() as u32 - 1u32) as usize;
            lucky_committee.push((committee[lucky_index].clone(), Vec::new()));
            committee.remove(lucky_index);
        }

        for i in 0..DISTRIBUTION as usize {
            let index = i % lucky_committee_num;
            lucky_committee[index].1.push(i);
        }

        Some(lucky_committee)
    }

    fn statistic_result() {
        let live_machines = <online_profile::Pallet<T>>::live_machines();
        let booked_machine = live_machines.booked_machine;
        let now = <frame_system::Module<T>>::block_number();

        for machine_id in booked_machine {
            let machine_committee = Self::machine_committee(machine_id.clone());
            // 当不为Summary状态时查看是否到了48小时，如果不到则返回
            if machine_committee.status != OCVerifyStatus::Summarizing {
                if now < machine_committee.book_time + SUBMIT_RAW_END.into() {
                    continue
                }
            }

            // committee, that should be slashed
            let mut slash_committee = Vec::new();
            // committee, that should be reward to
            let mut reward_committee = Vec::new();
            // committee, that finished his work, and should return his stake back
            let mut unstake_committee = Vec::new();

            match Self::summary_confirmation(&machine_id) {
                MachineConfirmStatus::Confirmed(summary) => {
                    slash_committee.extend(summary.unruly.clone());
                    slash_committee.extend(summary.against);
                    slash_committee.extend(summary.invalid_support);
                    unstake_committee.extend(summary.valid_support.clone());
                    if T::OCOperations::oc_confirm_machine(summary.valid_support.clone(), summary.info.unwrap()).is_ok()
                    {
                        let valid_support = summary.valid_support.clone();
                        for a_committee in valid_support {
                            // 如果机器成功上线，则从委员会确认的机器中删除，添加到成功上线的记录中
                            let mut committee_machine = Self::committee_machine(&a_committee);
                            if let Err(index) = committee_machine.online_machine.binary_search(&machine_id) {
                                committee_machine.online_machine.insert(index, machine_id.clone());
                            }
                            CommitteeMachine::<T>::insert(&a_committee, committee_machine);
                        }

                        let mut machine_committee = Self::machine_committee(&machine_id);
                        machine_committee.status = OCVerifyStatus::Finished;
                        machine_committee.onlined_committee = summary.valid_support;
                        MachineCommittee::<T>::insert(&machine_id, machine_committee);
                    }
                },
                MachineConfirmStatus::Refuse(summary) => {
                    slash_committee.extend(summary.unruly.clone());
                    slash_committee.extend(summary.invalid_support);
                    reward_committee.extend(summary.against.clone());
                    unstake_committee.extend(summary.against.clone());

                    let mut machine_committee = Self::machine_committee(&machine_id);
                    machine_committee.status = OCVerifyStatus::Finished;
                    MachineCommittee::<T>::insert(&machine_id, machine_committee);

                    let _ = T::OCOperations::oc_refuse_machine(machine_id.clone(), reward_committee);
                },
                MachineConfirmStatus::NoConsensus(summary) => {
                    slash_committee.extend(summary.unruly.clone());
                    unstake_committee.extend(machine_committee.confirmed_committee.clone());
                    let _ = Self::revert_book(machine_id.clone());

                    T::OCOperations::oc_revert_booked_machine(machine_id.clone());
                },
            }

            // 惩罚没有提交信息的委员会
            for a_committee in slash_committee {
                let committee_ops = Self::committee_ops(&a_committee, &machine_id);
                <T as pallet::Config>::ManageCommittee::add_slash(a_committee, committee_ops.staked_dbc, vec![]);
            }

            for a_committee in unstake_committee {
                let committee_ops = Self::committee_ops(&a_committee, &machine_id);
                let _ = <T as pallet::Config>::ManageCommittee::change_used_stake(
                    &a_committee,
                    committee_ops.staked_dbc,
                    false,
                );
            }

            // Do cleaning
            for a_committee in machine_committee.booked_committee {
                CommitteeOps::<T>::remove(&a_committee, &machine_id);
                MachineSubmitedHash::<T>::remove(&machine_id);

                // 改变committee_machine
                let mut committee_machine = Self::committee_machine(&a_committee);
                OCCommitteeMachineList::rm_one(&mut committee_machine.booked_machine, &machine_id);
                OCCommitteeMachineList::rm_one(&mut committee_machine.hashed_machine, &machine_id);
                OCCommitteeMachineList::rm_one(&mut committee_machine.confirmed_machine, &machine_id);

                CommitteeMachine::<T>::insert(&a_committee, committee_machine);
            }
        }
    }

    // 重新进行派单评估
    // 该函数将清除本模块信息，并将online_profile机器状态改为ocw_confirmed_machine
    // 清除信息： OCCommitteeMachineList, OCMachineCommitteeList, OCCommitteeOps
    fn revert_book(machine_id: MachineId) -> Result<(), ()> {
        let machine_committee = Self::machine_committee(&machine_id);

        // 给提交了信息的委员会退押金
        for booked_committee in machine_committee.confirmed_committee {
            let _committee_ops = Self::committee_ops(&booked_committee, &machine_id);
            // TODO: committee 提供
            // Self::reduce_stake(&booked_committee, committee_ops.staked_dbc)?;
        }

        // 清除预订了机器的委员会
        for booked_committee in machine_committee.booked_committee {
            CommitteeOps::<T>::remove(&booked_committee, &machine_id);

            let mut committee_machine = Self::committee_machine(&booked_committee);
            if let Ok(index) = committee_machine.booked_machine.binary_search(&machine_id) {
                committee_machine.booked_machine.remove(index);
            }
            if let Ok(index) = committee_machine.hashed_machine.binary_search(&machine_id) {
                committee_machine.hashed_machine.remove(index);
            }
            if let Ok(index) = committee_machine.confirmed_machine.binary_search(&machine_id) {
                committee_machine.confirmed_machine.remove(index);
            }
            CommitteeMachine::<T>::insert(booked_committee, committee_machine);
        }

        MachineCommittee::<T>::remove(&machine_id);
        Ok(())
    }

    // 总结机器的确认情况: 检查机器是否被确认，并检查提交的信息是否一致
    // 返回三种状态：
    // 1. 无共识：处理办法：退还委员会质押，机器重新派单。
    // 2. 支持上线: 处理办法：扣除所有反对上线，支持上线但提交无效信息的委员会的质押。
    // 3. 反对上线: 处理办法：反对的委员会平分支持的委员会的质押。扣5%矿工质押，允许矿工再次质押而上线。
    pub fn summary_confirmation(machine_id: &MachineId) -> MachineConfirmStatus<T::AccountId> {
        let machine_committee = Self::machine_committee(machine_id);

        let mut summary = Summary { ..Default::default() };
        // 支持的委员会可能提交不同的机器信息
        let mut uniq_machine_info: Vec<CommitteeUploadInfo> = Vec::new();
        // 不同机器信息对应的委员会
        let mut committee_for_machine_info = Vec::new();

        for a_committee in machine_committee.booked_committee {
            // 记录没有提交原始信息的委员会
            if machine_committee.confirmed_committee.binary_search(&a_committee).is_err() {
                summary.unruly.push(a_committee);
                continue
            }

            let a_machine_info = Self::committee_ops(a_committee.clone(), machine_id).machine_info;
            // 记录上反对上线的委员会
            if a_machine_info.is_support == false {
                summary.against.push(a_committee);
                continue
            }

            match uniq_machine_info.iter().position(|r| r == &a_machine_info) {
                None => {
                    uniq_machine_info.push(a_machine_info.clone());
                    committee_for_machine_info.push(vec![a_committee.clone()]);
                },
                Some(index) => committee_for_machine_info[index].push(a_committee),
            };
        }

        // 如果没有人提交确认信息，则无共识。返回分派了订单的委员会列表，对其进行惩罚
        if machine_committee.confirmed_committee.len() == 0 {
            return MachineConfirmStatus::NoConsensus(summary)
        }

        // 统计committee_for_machine_info中有多少委员会站队最多
        let support_committee_num: Vec<usize> = committee_for_machine_info.iter().map(|item| item.len()).collect();
        // 最多多少个委员会达成一致意见
        let max_support = support_committee_num.iter().max();

        match max_support {
            None => {
                // 如果没有支持者，且有反对者，则拒绝接入。
                if summary.against.len() > 0 {
                    return MachineConfirmStatus::Refuse(summary)
                }
                // 反对者支持者都为0
                return MachineConfirmStatus::NoConsensus(summary)
            },
            Some(max_support_num) => {
                // 多少个机器信息的支持等于最大的支持
                let max_support_group = support_committee_num.iter().filter(|n| n == &max_support_num).count();

                if max_support_group == 1 {
                    let committee_group_index =
                        support_committee_num.iter().position(|r| r == max_support_num).unwrap();

                    // 记录所有的无效支持
                    for index in 0..committee_for_machine_info.len() {
                        if index == committee_group_index {
                            continue
                        }
                        summary.invalid_support.extend(committee_for_machine_info[index].clone());
                    }

                    if summary.against.len() > *max_support_num {
                        // 反对多于支持
                        summary.invalid_support = committee_for_machine_info[committee_group_index].clone();
                        return MachineConfirmStatus::Refuse(summary)
                    } else if summary.against.len() == *max_support_num {
                        // 反对等于支持
                        summary.invalid_support = committee_for_machine_info[committee_group_index].clone();
                        return MachineConfirmStatus::NoConsensus(summary)
                    } else {
                        // 反对小于支持
                        // 记录上所有的有效支持
                        summary.valid_support = committee_for_machine_info[committee_group_index].clone();
                        summary.info = Some(uniq_machine_info[committee_group_index].clone());
                        return MachineConfirmStatus::Confirmed(summary)
                    }
                }

                // 如果有两组都是Max个委员会支, 则所有的支持都是无效的支持
                for index in 0..committee_for_machine_info.len() {
                    summary.invalid_support.extend(committee_for_machine_info[index].clone())
                }
                if summary.against.len() > *max_support_num {
                    return MachineConfirmStatus::Refuse(summary)
                }

                // against <= max_support 且 max_support_group > 1，且反对的不占多数
                return MachineConfirmStatus::NoConsensus(summary)
            },
        }
    }
}

// RPC
impl<T: Config> Module<T> {
    pub fn get_machine_committee_list(machine_id: MachineId) -> OCMachineCommitteeList<T::AccountId, T::BlockNumber> {
        Self::machine_committee(machine_id)
    }

    pub fn get_committee_machine_list(committee: T::AccountId) -> OCCommitteeMachineList {
        Self::committee_machine(committee)
    }

    pub fn get_committee_ops(
        committee: T::AccountId,
        machine_id: MachineId,
    ) -> RpcOCCommitteeOps<T::BlockNumber, BalanceOf<T>> {
        let oc_committee_ops = Self::committee_ops(&committee, &machine_id);
        let committee_info = Self::machine_committee(&machine_id);

        RpcOCCommitteeOps {
            booked_time: committee_info.book_time,
            staked_dbc: oc_committee_ops.staked_dbc,
            verify_time: oc_committee_ops.verify_time,
            confirm_hash: oc_committee_ops.confirm_hash,
            hash_time: oc_committee_ops.hash_time,
            confirm_time: oc_committee_ops.confirm_time,
            machine_status: oc_committee_ops.machine_status,
            machine_info: oc_committee_ops.machine_info,
        }
    }
}