// refer: https://polkadot.js.org/docs/substrate/storage

// Import the API & Provider and some utility functions
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

async function main() {
  // Construct
  const wsProvider = new WsProvider('wss://innertest.dbcwallet.io');

  // Create the API and wait until ready
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      "PriceURL": "Text",
      "URL": "Text",
      "MachineId": "Text",
      "OrderId": "u64",
      "LiveMachine": {
        "bonding_machine": "Vec<MachineId>",
        "ocw_confirmed_machine": "Vec<MachineId>",
        "booked_machine": "Vec<MachineId>",
        "waiting_hash": "Vec<MachineId>",
        "bonded_machine": "Vec<MachineId>"
      },
      "BlockNumber": "u32",
      "MachineInfo": {
        "machine_owner": "AccountId",
        "bonding_height": "BlockNumber",
        "machine_status": "MachineStatus",
        "ocw_machine_info": "OCWMachineInfo",
        "machine_grades": "u64",
        "committee_confirm": "BTreeMap<AccountId, CommitteeConfirmation",
        "reward_committee": "Vec<AccountId>",
        "reward_deadline": "BlockNumber"
      },
      "CommitteeConfirmation": {
        "committee": "AccountId",
        "confirm_time": "BlockNumber",
        "is_confirmed": "bool"
      },
      "MachineStatus": {
        "_enum": [
          "Bonding",
          "Booked",
          "WaitingHash",
          "Bonded",
          "WaitingFulfill"
        ]
      },
      "OCWMachineInfo": {
        "cpu": "CPU",
        "disk": "Disk",
        "gpu": "GPU",
        "ip": "Vec<u8>",
        "mem": "Vec<u8>",
        "os": "Vec<u8>",
        "version": "Vec<u8>"
      },
      "CPU": {
        "num": "Vec<u8>",
        "type": "Vec<u8>"
      },
      "Disk": {},
      "GPU": {
        "num": "Vec<u8>",
        "gpus": "Vec<GPUDetail>"
      },
      "GPUDetail": {
        "grade": "Vec<u8>"
      },
      "AccountInfo": {
        "nonce": "u32",
        "consumers": "u32",
        "providers": "u32",
        "data": "AccountData"
      },
      "StakingLedger": {
        "stash": "AccountId",
        "total": "Compact<Balance>",
        "active": "Compact<Balance>",
        "unlocking": "Vec<UnlockChunk>",
        "claimed_rewards": "Vec<EraIndex>",
        "released_rewards": "Balance",
        "upcoming_rewards": "Vec<Balance>"
      },
      "UnlockChunk": {
        "value": "Compact<Balance>",
        "era": "Compact<EraIndex>"
      },
      "CommitteeMachine": {
        "machine_id": "Vec<MachineId>",
        "total_calc_points": "u64",
        "total_gpu_num": "u64",
        "total_reward": "Balance"
      },
      "LCCommitteeMachineList": {
        "booked_machine": "Vec<(MachineId, BlockNumber)>",
        "hashed_machine": "Vec<MachineId>",
        "confirmed_machine": "Vec<MachineId>",
        "online_machine": "Vec<MachineId>"
      },
      "LCCommitteeMachineOps": {
        "booked_time": "BlockNumber",
        "confirm_hash": "[u8; 16]",
        "hash_time": "BlockNumber",
        "confirm_raw": "Vec<u8>",
        "confirm_time": "BlockNumber",
        "confirm_result": "bool",
        "machine_status": "MachineStatus"
      },
      "LCMachineCommitteeList": {
        "book_time": "BlockNumber",
        "booked_committee": "Vec<(AccountId, BlockNumber)>",
        "hashed_committee": "Vec<AccountId>",
        "confirm_start": "BlockNumber",
        "confirmed_committee": "Vec<AccountId>",
        "onlined_committee": "Vec<AccountId>"
      },
      "MachineStatus": {
        "_enum": [
          "Booked",
          "Hashed",
          "Confirmed"
        ]
      },
      "CommitteeMachineList": {
        "booked_order": "Vec<OrderId>",
        "hashed_order": "Vec<OrderId>",
        "confirmed_order": "Vec<Order>",
        "online_machine": "Vec<MachineId>"
      },
      "CommitteeMachineOps": {
        "booked_time": "BlockNumber",
        "encrypted_err_info": "Option<Vec<u8>>",
        "encrypted_login_info": "Option<Vec<u8>>",
        "encrypted_time": "BlockNumber",
        "confirm_hash": "Hash",
        "hash_time": "BlockNumber",
        "confirm_raw": "Vec<u8>",
        "confirm_time": "BlockNumber",
        "confirm_result": "bool",
        "machine_status": "MachineStatus"
      },
      "EraRewardBalance": {
        "total": "Balance",
        "individual": "BTreeMap<AccountId, Balance>"
      },
      "LiveOrderList": {
        "reported_order": "Vec<OrderId>",
        "fully_order": "Vec<OrderId>",
        "fully_reporter_hashed": "Vec<OrderId>",
        "fully_committee_hashed": "Vec<OrderId>",
        "fully_raw": "Vec<OrderId>"
      },
      "MachineCommitteeList": {
        "order_id": "OrderId",
        "report_time": "BlockNumber",
        "booked_committee": "Vec<AccountId>",
        "hashed_committee": "Vec<AccountId>",
        "confirm_start": "BlockNumber",
        "confirmed_committee": "Vec<AccountId>",
        "onlined_committee": "Vec<AccountId>"
      },
      "Pubkey": "Vec<u8>",
      "ReporterRecord": {
        "reported_id": "Vec<OrderId>"
      },
      "StakerList": {
        "committee": "Vec<AccountId>",
        "pubkey_list": "Vec<AccountId>",
        "fulfill_list": "Vec<AccountId>",
        "chill_list": "Vec<AccountId>",
        "black_list": "Vec<AccountId>",
      }
    }
  });

  const a = await api.query.onlineProfile.committeeLimit();
  console.log("### CommitteeLimit is: ", a);

  const b = await api.query.system.account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
  console.log("### Account info is: ", b);
  process.exit(0);
}

main().catch(console.error);
