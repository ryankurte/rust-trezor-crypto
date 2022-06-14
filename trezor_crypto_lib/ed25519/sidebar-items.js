initSidebarItems({"fn":[["dalek_curve25519_scalarmult","Scalar multiplication using the provided basepoint"],["dalek_curved25519_scalarmult_basepoint","Perform scalar multiplication of `e` over the edwards curve point"],["dalek_ed25519_publickey","Derives a public key from a private key (using the default `sha512` digest)"],["dalek_ed25519_publickey_ext","Generate a public key using the expanded (`sk + sk_ext`) form of the secret key"],["dalek_ed25519_randombytes_unsafe","Generate random bytes using the system RNG"],["dalek_ed25519_sign","Signs a message using the provided secret key (using the default `sha512` digest)"],["dalek_ed25519_sign_ext","Generate a signature using the expanded (`sk + sk_ext`) form of the secret key."],["dalek_ed25519_sign_open","Verifies a signed message (using the default `sha512` digest)"],["dalek_ed25519_sign_open_batch","Batch verify signatures, `valid[i] == 1` for valid, `valid[i] == 0` otherwise INCOMPLETE"]],"mod":[["consts","Common constants"],["keccak","ed25519 API using `keccak512` signatures, equivalent to `ed25519-donna` APIs generated with a custom `keccak512` hasher (see `tests/ed25519-keccak.c`)"],["sha3","ed25519 API using `sha3` signatures, equivalent to `ed25519-donna` APIs generated with a custom `sha3` hasher (see `tests/ed25519-sha3.c`)"]],"type":[["PublicKey","Ed25519 Public Key, compatible with donna’s `typedef unsigned char ed25519_public_key[32]`"],["Scalar","Ed25519 Scalar, compatible with donna’s `typedef unsigned char curved25519_key[32]`"],["SecretKey","Ed25519 Secret Key, compatible with donna’s `typedef unsigned char ed25519_secret_key[32]`"],["Signature","Ed25519 Signature, compatible with donna’s `typedef unsigned char ed25519_signature[64];`"]]});