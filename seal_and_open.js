// https://github.com/polkadot-js/common/issues/929

// For ed25519 pairs there is a seal/open pairing -
// https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/nacl/seal.ts
// https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/nacl/open.ts
// It is not available for sr25519 (see #633), nor ecdsa
