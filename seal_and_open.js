// https://github.com/polkadot-js/common/issues/929

// For ed25519 pairs there is a seal/open pairing -
// https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/nacl/seal.ts
// https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/nacl/open.ts
// It is not available for sr25519 (see #633), nor ecdsa

// https://github.com/polkadot-js/common/issues/929
// https://github.com/polkadot-js/common/issues/633

// 1. sr25519 to ed25519

import { naclSeal, naclOpen } from '@polkadot/util-crypto';
import { Keyring } from "@polkadot/keyring";

async function main() {
    // message: Uint8Array, senderBoxSecret: Uint8Array, receiverBoxPublic: Uint8Array, nonce: Uint8Array = randomAsU8a(24)
    // naclSeal([...], [...], [...], [...]);

  // known mnemonic, well, now it is - don't use it for funds
  const MNEMONIC =
    "sample split bamboo west visual approve brain fox arch impact relief smile";

    // type: sr25519, ssFormat: 42 (defaults)
    const keyring = new Keyring({ type: "ed25519" });
    const accountFromKeyring = keyring.createFromUri(MNEMONIC);

    const recipient = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

    var a = naclSeal(Buffer.from("Hello world"), Buffer.from(accountFromKeyring.key),Buffer.from(recipient));

    // sealed: Uint8Array, nonce: Uint8Array, senderBoxPublic: Uint8Array, receiverBoxSecret: Uint8Array
    // naclOpen([...], [...], [...]); // => [...]
}

main().catch((error) => console.log(error.message));
