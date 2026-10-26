# commoncrypto-rs

SHA-256 through Apple's `CommonCrypto` in `libSystem`. The library name is `commoncrypto`.
The crate has no dependencies. It supports Apple targets and compiles to an empty crate on other targets.

## SHA-256

`Sha256::new` starts a hash. `update` accepts a byte slice and preserves the state between calls.
It divides inputs larger than `u32::MAX` into separate `CommonCrypto` calls without a copy.
`finalize` consumes the state and returns a 32-byte digest.
Each method returns `std::io::Result` and reports a `CommonCrypto` failure.

```rust,ignore
use commoncrypto::Sha256;

let mut hash = Sha256::new()?;
hash.update(b"a")?;
hash.update(b"bc")?;
let digest = hash.finalize()?;
```

The `ffi` module exposes `CC_SHA256_CTX` and the three raw SHA-256 functions.
The crate does not implement other digests, encryption, HMAC, or key derivation.

## File example

```sh
cargo run --example sha256 -- file.mp4 poster.png
shasum -a 256 file.mp4 poster.png
```

The example reads each file in 64 KiB blocks and prints its lowercase hexadecimal digest.

## License

MIT OR Apache-2.0
