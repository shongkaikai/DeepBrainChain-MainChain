#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use lease_committee::{
    LCCommitteeMachineList, LCMachineCommitteeList, MachineId, RpcLCCommitteeOps,
};
use sp_runtime::traits::MaybeDisplay;
use sp_std::prelude::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime amalgamator file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait LcRpcApi<AccountId, BlockNumber> where
        AccountId: codec::Codec + Ord,
        BlockNumber: Codec + MaybeDisplay,
    {
        fn get_machine_committee_list(machine_id: MachineId) -> LCMachineCommitteeList<AccountId, BlockNumber>;
        fn get_committee_machine_list(committee: AccountId) -> LCCommitteeMachineList;
        fn get_committee_ops(committee: AccountId, machine_id: MachineId) -> RpcLCCommitteeOps<BlockNumber>;
    }
}