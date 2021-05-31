// refer: https://polkadot.js.org/docs/substrate/extrinsics

// 使用： node sign_txs.js
// --port="wss://innertest.dbcwallet.io"
// --module=onlineProfile --func=bondMachine
// --key="sample split bamboo west visual approve brain fox arch impact relief smile"

// Import the API & Provider and some utility functions
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import fs from "fs";
import minimist from "minimist";

async function dbcTest_sayHello(callFunc, accountFromKeyring, nonce) {
  const a = await callFunc().signAndSend(
    accountFromKeyring,
    { nonce },
    ({ events = [], status }) => {
      console.log("Tx_status:", status.type);

      if (status.isInBlock) {
        console.log("Tx_inBlock:", status.asInBlock.toHex());

        events.forEach(({ event: { data, method, section }, phase }) => {
          console.log(
            "Event:",
            phase.toString(),
            `${section}.${method}`,
            data.toString()
          );
        });
      } else if (status.isFinalized) {
        console.log("Finalized_block_hash:", status.asFinalized.toHex());

        process.exit(0);
      }
    }
  );
}

async function onlineProfile_bondMachine(api, accountFromKeyring, nonce) {}

async function main() {
  // 读取参数
  const args = minimist(process.argv.slice(2));

  // 构建连接
  const wsProvider = new WsProvider(args["port"]);
  const data = fs.readFileSync("types.json");
  const type_json = JSON.parse(data);
  // Create the API and wait until ready
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: type_json,
  });

  // 读取助记词 type: sr25519, ssFormat: 42 (defaults)
  const keyring = new Keyring({ type: "sr25519" });
  const accountFromKeyring = keyring.createFromUri(args["key"]);
  // 获取账户nonce
  const { nonce } = await api.query.system.account(accountFromKeyring.address);

  const onlineProfile_bondMachine = api.tx.dbcTesting.sayHello().signAndSend;

  // TODO: 根据模块与方法进行调用
  // const expr = "Papayas";
  switch (args["module"]) {
    case "onlineProfile":
      switch (args["func"]) {
        case "bondMachine":
          const callFunc = api.tx.dbcTesting.sayHello;
          await dbcTest_sayHello(callFunc, accountFromKeyring, nonce).catch(error => console.log(error.message));
          break;
      }

      break;
    case "dbcTesting":
      break;
    case "Papayas":
      break;
    default:
      console.log(`Sorry, we are out of ${expr}.`);
  }
}

// main().catch(console.error);
main().catch(error => console.log(error.message));
