use crate::CustomErr;
use codec::{Decode, Encode};
use dbc_support::{
    report::{MachineFaultType, ReportConfirmStatus, ReportStatus},
    verify_slash::OPSlashReason,
    ItemList, MachineId, RentOrderId, ReportHash, ReportId, FOUR_HOUR, THREE_HOUR, TWO_DAY,
};
use frame_support::ensure;
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::{ops::Sub, vec::Vec};

pub fn into_op_err<BlockNumber>(
    fault_type: &MachineFaultType,
    report_time: BlockNumber,
) -> OPSlashReason<BlockNumber> {
    match fault_type {
        MachineFaultType::RentedInaccessible(..) => OPSlashReason::RentedInaccessible(report_time),
        MachineFaultType::RentedHardwareMalfunction(..) =>
            OPSlashReason::RentedHardwareMalfunction(report_time),
        MachineFaultType::RentedHardwareCounterfeit(..) =>
            OPSlashReason::RentedHardwareCounterfeit(report_time),
        MachineFaultType::OnlineRentFailed(..) => OPSlashReason::OnlineRentFailed(report_time),
    }
}

// TODO: 合并该错误类型
// 报告的详细信息
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct IRReportInfoDetail<AccountId, BlockNumber, Balance> {
    ///报告人
    pub reporter: AccountId,
    /// 报告提交时间
    pub report_time: BlockNumber,
    /// 报告人质押数量
    pub reporter_stake: Balance,
    /// 第一个委员会抢单时间
    pub first_book_time: BlockNumber,
    /// 出问题的机器，只有委员会提交原始信息时才存入
    pub machine_id: MachineId,
    /// 出问题的机器的租用ID
    pub rent_order_id: RentOrderId,
    /// 机器的故障原因
    pub err_info: Vec<u8>,
    /// 当前正在验证机器的委员会
    pub verifying_committee: Option<AccountId>,
    /// 抢单的委员会
    pub booked_committee: Vec<AccountId>,
    /// 获得报告人提交了加密信息的委员会列表
    pub get_encrypted_info_committee: Vec<AccountId>,
    /// 提交了检查报告Hash的委员会
    pub hashed_committee: Vec<AccountId>,
    /// 开始提交raw信息的时间
    pub confirm_start: BlockNumber,
    /// 提交了Raw信息的委员会
    pub confirmed_committee: Vec<AccountId>,
    /// 支持报告人的委员会
    pub support_committee: Vec<AccountId>,
    /// 不支持报告人的委员会
    pub against_committee: Vec<AccountId>,
    /// 当前报告的状态
    pub report_status: ReportStatus,
    /// 机器的故障类型
    pub machine_fault_type: MachineFaultType,
}

impl<Account, BlockNumber, Balance> IRReportInfoDetail<Account, BlockNumber, Balance>
where
    Account: Default + Clone + Ord,
    BlockNumber:
        Default + PartialEq + Zero + From<u32> + Copy + Sub<Output = BlockNumber> + PartialOrd,
    Balance: Default,
{
    pub fn new(
        reporter: Account,
        report_time: BlockNumber,
        machine_fault_type: MachineFaultType,
        reporter_stake: Balance,
    ) -> Self {
        let report_info = IRReportInfoDetail {
            reporter,
            report_time,
            machine_fault_type: machine_fault_type.clone(),
            reporter_stake,
            ..Default::default()
        };

        // // 该类型错误可以由程序快速完成检测，因此可以提交并需记录machine_id
        // if let IRMachineFaultType::RentedInaccessible(machine_id, rent_order_id) =
        //     machine_fault_type.clone()
        // {
        //     report_info.machine_id = machine_id;
        //     report_info.rent_order_id = rent_order_id;
        // }

        report_info
    }

    pub fn can_book(&self, committee: &Account) -> Result<(), CustomErr> {
        // 检查订单是否可以抢定
        ensure!(self.report_time != Zero::zero(), CustomErr::ReportNotAllowBook);
        ensure!(
            matches!(self.report_status, ReportStatus::Reported | ReportStatus::WaitingBook),
            CustomErr::ReportNotAllowBook
        );
        ensure!(self.booked_committee.len() < 3, CustomErr::ReportNotAllowBook);
        ensure!(self.booked_committee.binary_search(committee).is_err(), CustomErr::AlreadyBooked);
        Ok(())
    }

    pub fn book_report(&mut self, committee: Account, now: BlockNumber) {
        ItemList::add_item(&mut self.booked_committee, committee.clone());

        if self.report_status == ReportStatus::Reported {
            // 是第一个预订的委员会时:
            self.first_book_time = now;
            self.confirm_start = now + THREE_HOUR.into();
            // self.confirm_start = match self.machine_fault_type {
            //     // 将在5分钟后开始提交委员会的验证结果
            //     MachineFaultType::RentedInaccessible(..) => now + 10u32.into(),
            //     // 将在三个小时之后开始提交委员会的验证结果
            //     _ => now + THREE_HOUR.into(),
            // };
        }

        self.report_status = ReportStatus::Verifying;
        self.verifying_committee = Some(committee);

        // self.report_status = match self.machine_fault_type {
        //     MachineFaultType::RentedInaccessible(..) =>
        //         if self.booked_committee.len() == 3 {
        //             ReportStatus::Verifying
        //         } else {
        //             ReportStatus::WaitingBook
        //         },
        //     _ => {
        //         // 仅在不是RentedInaccessible时进行记录，因为这些情况只能一次有一个验证委员会
        //         self.verifying_committee = Some(committee);
        //         // 改变report状态为正在验证中，此时禁止其他委员会预订
        //         ReportStatus::Verifying
        //     },
        // };
    }

    pub fn can_submit_encrypted_info(&self, from: &Account, to: &Account) -> Result<(), CustomErr> {
        // ensure!(
        //     !matches!(self.machine_fault_type, MachineFaultType::RentedInaccessible(..)),
        //     CustomErr::NotNeedEncryptedInfo
        // );
        ensure!(&self.reporter == from, CustomErr::NotOrderReporter);
        ensure!(self.report_status == ReportStatus::Verifying, CustomErr::OrderStatusNotFeat);
        ensure!(self.booked_committee.binary_search(to).is_ok(), CustomErr::NotOrderCommittee);
        Ok(())
    }

    pub fn can_summary_fault(&self) -> Result<(), ()> {
        // 忽略掉线的类型
        if self.first_book_time == Zero::zero()
        // || matches!(self.machine_fault_type, IRMachineFaultType::RentedInaccessible(..))
        {
            return Err(())
        }

        Ok(())
    }

    // Other fault type
    pub fn can_summary(&self, now: BlockNumber) -> bool {
        if self.first_book_time == Zero::zero() {
            return false
        }

        // // 禁止对快速报告进行检查，快速报告会处理这种情况
        // if let IRMachineFaultType::RentedInaccessible(..) = self.machine_fault_type {
        //     return false
        // }

        // 未全部提交了原始信息且未达到了四个小时，需要继续等待
        if now - self.first_book_time < FOUR_HOUR.into() &&
            self.hashed_committee.len() != self.confirmed_committee.len()
        {
            return false
        }

        true
    }

    // Summary committee's handle result depend on support & against votes
    pub fn summary(&self) -> ReportConfirmStatus<Account> {
        if self.confirmed_committee.is_empty() {
            return ReportConfirmStatus::NoConsensus
        }

        if self.support_committee.len() >= self.against_committee.len() {
            return ReportConfirmStatus::Confirmed(
                self.support_committee.clone(),
                self.against_committee.clone(),
                self.err_info.clone(),
            )
        }
        ReportConfirmStatus::Refuse(self.support_committee.clone(), self.against_committee.clone())
    }
}

// 处理除了inaccessible错误之外的错误
impl<Account, BlockNumber, Balance> IRReportInfoDetail<Account, BlockNumber, Balance>
where
    Account: Default + Clone + Ord,
    BlockNumber:
        Default + PartialEq + Zero + From<u32> + Copy + Sub<Output = BlockNumber> + PartialOrd,
    Balance: Default,
{
    // 机器正在被该委员会验证，但该委员会超时未提交验证hash
    pub fn clean_not_submit_hash_committee(&mut self, verifying_committee: &Account) {
        self.verifying_committee = None;
        // 删除，以允许其他委员会进行抢单
        ItemList::rm_item(&mut self.booked_committee, verifying_committee);
        ItemList::rm_item(&mut self.get_encrypted_info_committee, verifying_committee);

        // 如果此时booked_committee.len() == 0；返回到最初始的状态，并允许取消报告
        if self.booked_committee.is_empty() {
            self.first_book_time = Zero::zero();
            self.confirm_start = Zero::zero();
            self.report_status = ReportStatus::Reported;
        } else {
            self.report_status = ReportStatus::WaitingBook
        };
    }

    // 当块高从抢单验证变为提交原始值时，移除最后一个正在验证的委员会
    pub fn clean_not_submit_raw_committee(&mut self, verifying_committee: &Account) {
        // 将最后一个委员会移除，不惩罚
        self.verifying_committee = None;
        ItemList::rm_item(&mut self.booked_committee, &verifying_committee);
        ItemList::rm_item(&mut self.get_encrypted_info_committee, &verifying_committee);
    }
}

/// 委员会抢单之后，对应订单的状态
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum IROrderStatus {
    /// 预订报告，状态将等待加密信息
    WaitingEncrypt,
    /// 获得加密信息之后，状态将等待加密信息
    Verifying,
    /// 等待提交原始信息
    WaitingRaw,
    /// 委员会已经完成了全部操作
    Finished,
}

impl Default for IROrderStatus {
    fn default() -> Self {
        Self::Verifying
    }
}

/// 委员会对报告的操作信息
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct IRCommitteeReportOpsDetail<BlockNumber, Balance> {
    pub booked_time: BlockNumber,
    /// reporter 提交的加密后的信息
    pub encrypted_err_info: Option<Vec<u8>>,
    pub encrypted_time: BlockNumber,
    pub confirm_hash: ReportHash,
    pub hash_time: BlockNumber,
    /// 委员会可以补充额外的信息
    pub extra_err_info: Vec<u8>,
    /// 委员会提交raw信息的时间
    pub confirm_time: BlockNumber,
    pub confirm_result: bool,
    pub staked_balance: Balance,
    pub order_status: IRReportOrderStatus,
}

impl<BlockNumber, Balance> IRCommitteeReportOpsDetail<BlockNumber, Balance> {
    pub fn add_encry_info(&mut self, info: Vec<u8>, time: BlockNumber) {
        self.encrypted_err_info = Some(info);
        self.encrypted_time = time;
        self.order_status = IRReportOrderStatus::Verifying;
    }

    pub fn book_report(
        &mut self,
        _fault_type: MachineFaultType,
        now: BlockNumber,
        order_stake: Balance,
    ) {
        // 更改committee_ps
        self.booked_time = now;
        self.order_status = IRReportOrderStatus::WaitingEncrypt;
        self.staked_balance = order_stake;

        // self.order_status = match fault_type {
        //     IRMachineFaultType::RentedInaccessible(..) => IRReportOrderStatus::Verifying,
        //     _ => {
        //         self.staked_balance = order_stake;
        //         IRReportOrderStatus::WaitingEncrypt
        //     },
        // };
    }

    pub fn can_submit_hash(&self) -> Result<(), CustomErr> {
        ensure!(self.order_status == IRReportOrderStatus::Verifying, CustomErr::OrderStatusNotFeat);
        Ok(())
    }

    pub fn add_hash(&mut self, hash: ReportHash, time: BlockNumber) {
        self.confirm_hash = hash;
        self.hash_time = time;
        self.order_status = IRReportOrderStatus::WaitingRaw;
    }
    pub fn add_raw(&mut self, time: BlockNumber, is_support: bool, extra_err_info: Vec<u8>) {
        self.confirm_time = time;
        self.extra_err_info = extra_err_info;
        self.confirm_result = is_support;
        self.order_status = IRReportOrderStatus::Finished;
    }
}

/// 委员会抢单之后，对应订单的状态
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum IRReportOrderStatus {
    /// 预订报告，状态将等待加密信息
    WaitingEncrypt,
    /// 获得加密信息之后，状态将等待加密信息
    Verifying,
    /// 等待提交原始信息
    WaitingRaw,
    /// 委员会已经完成了全部操作
    Finished,
}

impl Default for IRReportOrderStatus {
    fn default() -> Self {
        Self::Verifying
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum IRReportResultType {
    ReportSucceed,
    ReportRefused,
    ReporterNotSubmitEncryptedInfo,
    NoConsensus,
}

impl Default for IRReportResultType {
    fn default() -> Self {
        Self::ReportRefused
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum IRReportSlashResult {
    Pending,
    Canceled,
    Executed,
}

impl Default for IRReportSlashResult {
    fn default() -> Self {
        Self::Pending
    }
}

/// 报告的处理结果
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct IRReportResultInfo<AccountId, BlockNumber, Balance> {
    pub report_id: ReportId,
    pub reporter: AccountId,
    pub reporter_stake: Balance,

    pub inconsistent_committee: Vec<AccountId>,
    pub unruly_committee: Vec<AccountId>,
    pub reward_committee: Vec<AccountId>,
    pub committee_stake: Balance,

    pub machine_stash: AccountId,
    pub machine_id: MachineId,

    pub slash_time: BlockNumber,
    pub slash_exec_time: BlockNumber,
    pub report_result: IRReportResultType,
    pub slash_result: IRReportSlashResult,
}

impl<Account, Block, Balance> IRReportResultInfo<Account, Block, Balance>
where
    Account: Ord + Clone,
    Balance: Zero,
{
    // 接收report_info.summary结果，修改自身
    // 仅仅在summary_waiting_raw中使用
    pub fn get_verify_result(
        &mut self,
        now: Block,
        report_id: ReportId,
        committee_order_stake: Balance,
        report_info: &IRReportInfoDetail<Account, Block, Balance>,
    ) where
        Account: Default + Clone + Ord,
        Block: Default + PartialEq + Zero + From<u32> + Copy + Sub<Output = Block> + PartialOrd,
        Balance: Default + Copy,
    {
        self.report_id = report_id;
        self.slash_result = IRReportSlashResult::Pending;
        self.slash_time = now;
        self.slash_exec_time = now + TWO_DAY.into();
        self.reporter = report_info.reporter.clone();
        self.committee_stake = committee_order_stake;

        let verify_summary = report_info.summary();
        match verify_summary {
            // 报告成功
            ReportConfirmStatus::Confirmed(support, against, _) => {
                self.report_result = IRReportResultType::ReportSucceed;
                self.reporter_stake = report_info.reporter_stake;

                for a_committee in against {
                    ItemList::add_item(&mut self.inconsistent_committee, a_committee.clone());
                }

                for a_committee in support.clone() {
                    ItemList::add_item(&mut self.reward_committee, a_committee.clone());
                }
            },
            // 报告失败
            ReportConfirmStatus::Refuse(support_committee, against_committee) => {
                self.report_result = IRReportResultType::ReportRefused;
                self.reporter_stake = report_info.reporter_stake;

                // Slash support committee and release against committee stake
                self.i_exten_sorted(support_committee);
                self.r_exten_sorted(against_committee);
            },
            // 如果没有人提交，会出现NoConsensus的情况，并重新派单
            ReportConfirmStatus::NoConsensus => {
                self.report_result = IRReportResultType::NoConsensus;

                // 记录unruly的委员会，两天后进行惩罚
                ItemList::expand_to_order(
                    &mut self.unruly_committee,
                    report_info.booked_committee.clone(),
                );

                // 重新举报时，记录报告人的质押将被重新使用，因此不再退还。
                self.reporter_stake = Zero::zero();
            },
        }
    }

    fn i_exten_sorted(&mut self, a_list: Vec<Account>) {
        for a_item in a_list {
            ItemList::add_item(&mut self.inconsistent_committee, a_item);
        }
    }

    fn r_exten_sorted(&mut self, a_list: Vec<Account>) {
        for a_item in a_list {
            ItemList::add_item(&mut self.reward_committee, a_item);
        }
    }
}
