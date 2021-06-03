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

async function main() {
  // 读取参数
  const args = minimist(process.argv.slice(2), {string: ["key"]});
 
  // 构建连接
  const wsProvider = new WsProvider(args["port"]);
  const data = fs.readFileSync("types.json");
  const type_json = JSON.parse(data);
  // Create the API and wait until ready
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: type_json,
  });
 
  // 读取密钥 type: sr25519, ssFormat: 42 (defaults)
  const keyring = new Keyring({ type: "sr25519" });
  // const accountFromKeyring = keyring.createFromUri(args["key"]); // 从助记词生成账户
  const accountFromKeyring = keyring.addFromUri(args["key"]); // 从私钥生成账户对
  
  // 获取账户nonce
  const { nonce } = await api.query.system.account(accountFromKeyring.address);
 
  // 根据模块与方法进行调用
  switch (args["module"]) {
    case "onlineProfile":
      switch (args["func"]) {
        case "bondMachine":
          const callFunc = api.tx.onlineProfile.bondMachine;
          await do_sign_tx(
            callFunc,
            accountFromKeyring,
            nonce,
            ...args._
          ).catch((error) => console.log(error.message));
          break;
      }
      break;
    case "dbcTesting":
      switch (args["func"]) {
        case "sayHello":
          const callFunc = api.tx.dbcTesting.sayHello;
          await do_sign_tx(
            callFunc,
            accountFromKeyring,
            nonce,
            ...args._
          ).catch((error) => console.log(error.message));
          break;
      }
      break;
    case "Papayas":
      break;
    default:
      console.log(`Sorry, we are out of ${expr}.`);
  }
}

async function do_sign_tx(callFunc, accountFromKeyring, nonce, ...args) {
  const a = await callFunc(...args).signAndSend(
    accountFromKeyring,
    { nonce },
    ({ events = [], status }) => {
      console.log(`{"Tx_status:":"${status.type}"}`);

      if (status.isInBlock) {
        console.log(`{"Tx_inBlock":"${status.asInBlock.toHex()}"}`);

        events.forEach(({ event: { data, method, section }, phase }) => {
          console.log(
            `{"Event":${phase.toString()},"func":"${section}.${method}","data":${data.toString()}}`
          );
        });
      } else if (status.isFinalized) {
        console.log(`{"Finalized_block_hash:":"${status.asFinalized.toHex()}"}`);

        process.exit(0);
      }
    }
  );
}

// main().catch(console.error);
main().catch((error) => console.log(error.message));
