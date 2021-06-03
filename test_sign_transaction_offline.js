import { Keyring, ApiPromise, WsProvider } from "@polkadot/api";
import fs from "fs";
import fetch from "node-fetch";
import { TypeRegistry } from "@polkadot/types";
import { EXTRINSIC_VERSION } from "@polkadot/types/extrinsic/v4/Extrinsic";
import { cryptoWaitReady } from '@polkadot/util-crypto';

import {
  createSignedTx,
  createSigningPayload,
  decode,
  deriveAddress,
  getTxHash,
  methods,
  WESTEND_SS58_FORMAT,
} from "@substrate/txwrapper";

async function main() {

  // Wait for the promise to resolve async WASM
  await cryptoWaitReady();

  // Create Substrate's type registry. If you're using a custom chain, you can
  // also put your own types here.
  const registry = new TypeRegistry();

  const keyring = new Keyring({ type: "sr25519" });
  const private_key =
    "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a";
  const account_key_pair = keyring.addFromUri(private_key); // 从私钥生成账户对

  const { block } = await rpcToNode("chain_getBlock");
  const blockHash = await rpcToNode("chain_getBlockHash");
  const genesisHash = await rpcToNode("chain_getBlockHash", [0]);
  const metadataRpc = await rpcToNode("state_getMetadata");
  const { specVersion, transactionVersion } = await rpcToNode(
    "state_getRuntimeVersion"
  );

  const unsigned = methods.balances.transfer(
    {
      value: 12,
      dest: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", // Bob
    },
    {
      address: deriveAddress(account_key_pair.publicKey, WESTEND_SS58_FORMAT),
      blockHash,
      blockNumber: registry
        .createType("BlockNumber", block.header.number)
        .toNumber(),
      eraPeriod: 64,
      genesisHash,
      metadataRpc,
      nonce: 0, // Assuming this is Alice's first tx on the chain
      specVersion,
      tip: 0,
      transactionVersion,
    },
    {
      metadataRpc,
      registry,
    }
  );

  // Construct the signing payload from an unsigned transaction.
  const signingPayload = createSigningPayload(unsigned, {
    registry,
  });

  // Sign a payload. This operation should be performed on an offline device.
  const signature = signWith(account_key_pair, signingPayload, {
    metadataRpc,
    registry,
  });

  // Serialize a signed transaction.
  const tx = createSignedTx(unsigned, signature, { metadataRpc, registry });
  console.log(`\nTransaction to Submit: ${tx}`);

  // Derive the tx hash of a signed transaction offline.
  const expectedTxHash = getTxHash(tx);

  // Send the tx to the node. Again, since `txwrapper` is offline-only, this
  // operation should be handled externally. Here, we just send a JSONRPC
  // request directly to the node.
  const actualTxHash = await rpcToNode("author_submitExtrinsic", [tx]);
  console.log(`ExpectedTxHash: ${expectedTxHash}, Actual Tx Hash: ${actualTxHash}`);

  process.exit(0);
}

main().catch((error) => console.log(error.message));

export function signWith(pair, signingPayload, options) {
  const { registry, metadataRpc } = options;
  // Important! The registry needs to be updated with latest metadata, so make
  // sure to run `registry.setMetadata(metadata)` before signing.
  // registry.setMetadata(createMetadata(registry, metadataRpc));

  const { signature } = registry
    .createType("ExtrinsicPayload", signingPayload, {
      version: EXTRINSIC_VERSION,
    })
    .sign(pair);

  return signature;
}

export function rpcToNode(method, params) {
  return fetch("http://127.0.0.1:9933", {
    body: JSON.stringify({
      id: 1,
      jsonrpc: "2.0",
      method,
      params,
    }),
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
  })
    .then((response) => response.json())
    .then(({ error, result }) => {
      if (error) {
        throw new Error(
          `${error.code} ${error.message}: ${JSON.stringify(error.data)}`
        );
      }

      return result;
    });
}
