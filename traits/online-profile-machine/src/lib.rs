#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;

// lease-committee_ops
pub trait LCOps {
    type AccountId;
    type MachineId;
    type CommitteeUploadInfo;

    fn lc_booked_machine(id: Self::MachineId);
    fn lc_revert_booked_machine(id: Self::MachineId);

    fn lc_confirm_machine(
        who: Vec<Self::AccountId>,
        machine_info: Self::CommitteeUploadInfo,
    ) -> Result<(), ()>;
    fn lc_refuse_machine(who: Vec<Self::AccountId>, id: Self::MachineId) -> Result<(), ()>;
}

pub trait OCWOps {
    type AccountId;
    type MachineId;

    // fn ocw_clean_booking_machine();
    fn rm_booked_id(id: &Self::MachineId);
    fn add_ocw_confirmed_id(id: Self::MachineId, wallet: Self::AccountId);
}

pub trait RTOps {
    type MachineId;
    type MachineStatus;

    fn change_machine_status(machine_id: &Self::MachineId, new_status: Self::MachineStatus);
}
