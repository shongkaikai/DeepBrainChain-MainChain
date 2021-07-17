import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import fs from "fs";
import minimist from "minimist";

async function main() {
  // 读取参数
  const args = minimist(process.argv.slice(2));

  // 构建连接
  const wsProvider = new WsProvider(args["port"]);
  const type_path = fs.readFileSync(args["type-file"]);
  const type_json = JSON.parse(type_path);
  const rpc_path = fs.readFileSync(args["rpc-file"]);
  const rpc_json = JSON.parse(rpc_path);

  // Create the API and wait until ready
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: type_json,
    rpc: rpc_json,
  });

  var funcMap = {};
  funcMap["staking"] = {};
  funcMap["staking"]["ledger"] = api.query.staking.ledger;

  funcMap["system"] = {};
  funcMap["system"]["account"] = api.query.system.account;

  funcMap["dbcPriceOcw"] = {};
  funcMap["dbcPriceOcw"]["avgPrice"] = api.query.dbcPriceOcw.avgPrice;
  funcMap["dbcPriceOcw"]["priceURL"] = api.query.dbcPriceOcw.priceURL;

  funcMap["committee"] = {};
  funcMap["committee"]["committeeStakeDBCPerOrder"] = api.query.committee.committeeStakeDBCPerOrder;

  funcMap["genericFunc"] = {};
  funcMap["genericFunc"]["fixedTxFee"] = api.query.genericFunc.fixedTxFee;

  funcMap["onlineProfile"] = {};
  funcMap["onlineProfile"]["stakePerGPU"] = api.query.onlineProfile.stakePerGPU;
  funcMap["onlineProfile"]["sysInfo"] = api.query.onlineProfile.sysInfo;

  var callFunc = funcMap[args["module"]][args["func"]];
  await do_query(callFunc, ...args._).catch((error) =>
    console.log(error.message)
  );
}

async function do_query(callFunc, ...args) {
  const a = await callFunc(...args);
  // https://polkadot.js.org/docs/api/start/types.basics/
  // console.log(JSON.stringify(a.toHuman()));
  // console.log(a.toString());
  console.log(a.toJSON());
  // console.log(a.data.free.toString());

  process.exit(0);
}

main().catch((error) => console.log(error.message));
