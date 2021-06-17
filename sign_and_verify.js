import { stringToU8a, u8aToHex } from "@polkadot/util";
import { cryptoWaitReady, signatureVerify } from "@polkadot/util-crypto";
import { Keyring } from "@polkadot/keyring";

import {
  naclSeal,
  naclOpen,
  naclKeypairFromSeed,
  naclKeypairFromString,
  fromPath
} from "@polkadot/util-crypto";

await cryptoWaitReady();

const keyring = new Keyring({ type: "sr25519" });
// const keyring = new Keyring({ type: "ed25519" });
// create Alice based on the development seed
const alice = keyring.addFromUri("//Alice");

console.log(alice.address);

// create the message, actual signature and verify
const message = stringToU8a("this is our message 12345");
const signature = alice.sign(message);
const isValid = alice.verify(message, signature);

// output the result
console.log(`${u8aToHex(signature)} is ${isValid ? "valid" : "invalid"}`);

// create the message and sign it
// const message = stringToU8a('this is our message');

const signature2 = alice.sign(message);

// verify the message using Alice's address
const { isValid2 } = signatureVerify(message, signature, alice.address);

// output the result
console.log(`${u8aToHex(signature2)} is ${isValid2 ? "valid" : "invalid"}`);

// naclSign

const sender = naclKeypairFromString("sender");
const receiver = naclKeypairFromString("receiver");

// Make id key for identifying each other
const senderIdKey = fromPath(
  sender,
  keyExtractPath("//1//1//1//1").path,
  "ed25519"
);
const receiverIdKey = fromPath(
  receiver,
  keyExtractPath("//2//2//2//2").path,
  "ed25519"
);

// Receiver sends encrypting public key to receive message to decrypt with his private key
const receiverIdBoxKey = naclBoxKeypairFromSecret(receiverIdKey.secretKey);

console.log(
  `Receiver sends receiver's public key to sender ${receiverIdBoxKey.publicKey.toString()}`
);

// Sender encrypts message to send with the public key the receiver sent and send it to receiver
const senderIdBoxKey = naclBoxKeypairFromSecret(senderIdKey.secretKey);
const message2 = new Uint8Array([1, 2, 3, 4, 5, 4, 3, 2, 1]);
const { nonce, sealed } = naclSeal(
  message2,
  senderIdBoxKey.secretKey,
  receiverIdBoxKey.publicKey
);

console.log(
  `Sender sends encrypted message to receiver ${sealed.toString()}, ${nonce.toString()}`
);

// Receiver opens encrypted message from the sender
// Question: How did the receiver get the senderIdBoxKey.publicKey? Is this even necessary?
let opened = naclOpen(
  sealed,
  nonce,
  senderIdBoxKey.publicKey,
  receiverIdBoxKey.secretKey
);
