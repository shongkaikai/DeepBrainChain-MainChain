import { stringToU8a, u8aToHex } from "@polkadot/util";
import { cryptoWaitReady, signatureVerify } from "@polkadot/util-crypto";
import { Keyring } from "@polkadot/keyring";

import {
  naclSeal,
  naclOpen,
  naclKeypairFromSeed,
  naclKeypairFromString,
  naclBoxKeypairFromSecret,
  deriveAddress,
} from "@polkadot/util-crypto";

await cryptoWaitReady();

const keyring = new Keyring({ type: "sr25519" });
const alice = keyring.addFromUri("//Alice");

// create the message, actual signature and verify
const message = stringToU8a("this is our message 12345");
const signature = alice.sign(message);

// verify the message using Alice's address
// const isValid = alice.verify(message, signature);
const { isValid } = signatureVerify(message, signature, alice.address);

// output the result
console.log(`${u8aToHex(signature)} is ${isValid ? "valid" : "invalid"}`);

console.log(`###Signature: ${u8aToHex(signature)}\n###Message: ${u8aToHex(message)},\n###Sender: ${alice.address}`);

// naclSign
const sender = naclKeypairFromString("sender");
const senderIdBoxKey = naclBoxKeypairFromSecret(sender.secretKey);

const receiver = naclKeypairFromString("receiver");
const receiverIdBoxKey = naclBoxKeypairFromSecret(receiver.secretKey);

console.log("sender addr", u8aToHex(sender.publicKey)); // ed25519 pubkey
console.log("sender seed", u8aToHex(sender.secretKey)); // same seed

// 在js中，是调用tweetnacl-js里的crypto_scalarmult方法,  crypto_scalarmult(q, n, _9); 其中，q为返回值为新生成的公钥
// n为私钥。对应C++版本：https://nacl.cr.yp.to/scalarmult.html
console.log("senderBoxPubKey", u8aToHex(senderIdBoxKey.publicKey));
console.log("senderBoxPriv", u8aToHex(senderIdBoxKey.secretKey)); // same seed

const keyring2 = new Keyring({ type: "sr25519" });
console.log("### Encode pubkey",keyring2.encodeAddress(senderIdBoxKey.publicKey));

// Receiver sends encrypting public key to receive message to decrypt with his private key
console.log(`Receiver sends receiver's public key to sender ${receiverIdBoxKey.publicKey.toString()}`);

// Sender encrypts message to send with the public key the receiver sent and send it to receiver
const message2 = new Uint8Array([1, 2, 3, 2, 1]);
const { nonce, sealed } = naclSeal(message2,senderIdBoxKey.secretKey, receiverIdBoxKey.publicKey);
console.log(`Sender sends encrypted message to receiver ${sealed.toString()}, ${nonce.toString()}`);
// Receiver opens encrypted message from the sender
let opened = naclOpen(sealed, nonce, senderIdBoxKey.publicKey, receiverIdBoxKey.secretKey);
console.log("Opened message is:", opened);
