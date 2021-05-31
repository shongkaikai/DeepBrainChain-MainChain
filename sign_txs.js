// refer: https://polkadot.js.org/docs/substrate/extrinsics

// Import the API & Provider and some utility functions
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import fs from "fs";

async function main() {
  // Construct
  const wsProvider = new WsProvider("wss://innertest.dbcwallet.io");

  var data = fs.readFileSync("types.json");
  var type_json = JSON.parse(data);

  // Create the API and wait until ready
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: type_json,
  });

  // known mnemonic, well, now it is - don't use it for funds
  const MNEMONIC =
    "sample split bamboo west visual approve brain fox arch impact relief smile";

  // type: sr25519, ssFormat: 42 (defaults)
  const keyring = new Keyring({ type: "sr25519" });
  const accountFromKeyring = keyring.createFromUri(MNEMONIC);
  // Get the nonce for the admin key
  const { nonce } = await api.query.system.account(accountFromKeyring.address);

  const a = await api.tx.dbcTesting
    .sayHello()
    .signAndSend(accountFromKeyring, { nonce }, ({ events = [], status }) => {
      console.log("Transaction status:", status.type);

      if (status.isInBlock) {
        console.log("Included at block hash", status.asInBlock.toHex());
        console.log("Events:");

        events.forEach(({ event: { data, method, section }, phase }) => {
          console.log(
            "\t",
            phase.toString(),
            `: ${section}.${method}`,
            data.toString()
          );
        });
      } else if (status.isFinalized) {
        console.log("Finalized block hash", status.asFinalized.toHex());

        process.exit(0);
      }
    });
}

main().catch(console.error);
