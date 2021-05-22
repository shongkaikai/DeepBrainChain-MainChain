// Import
import { ApiPromise, WsProvider } from '@polkadot/api';

// Construct
const wsProvider = new WsProvider('wss://innertest.dbcwallet.io');
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
      "_enum": ["Bonding", "Booked", "WaitingHash", "Bonded", "WaitingFulfill"]
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
    }
  }
});

// Do something
console.log(api.genesisHash.toHex());
