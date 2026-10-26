# Coverage

Library: CommonCrypto in `libSystem`.
Scope: SHA-256 only. The crate does not cover the complete CommonCrypto API.

| CommonCrypto API | Rust surface |
| --- | --- |
| `CC_SHA256_CTX` | `ffi::CC_SHA256_CTX` |
| `CC_SHA256_Init` | `Sha256::new`, `ffi::CC_SHA256_Init` |
| `CC_SHA256_Update` | `Sha256::update`, `ffi::CC_SHA256_Update` |
| `CC_SHA256_Final` | `Sha256::finalize`, `ffi::CC_SHA256_Final` |

The one-call `CC_SHA256` function is not bound.
MD2, MD4, MD5, SHA-1, SHA-224, SHA-384, and SHA-512 are not bound.
Encryption, HMAC, key derivation, and other CommonCrypto facilities are not bound.
