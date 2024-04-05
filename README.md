Running `cargo test` fails with:

```
error[E0308]: mismatched types
   --> /Users/alexfertel/.cargo/git/checkouts/foundry-f7cca724e93059b0/dfab23e/crates/linking/src/lib.rs:172:36
    |
172 |         Ok(LinkOutput { libraries, libs_to_deploy })
    |                                    ^^^^^^^^^^^^^^ expected `alloy_primitives::Bytes`, found `alloy_primitives::bytes_::Bytes`
    |
    = note: `alloy_primitives::bytes_::Bytes` and `alloy_primitives::Bytes` have similar names, but are actually distinct types
note: `alloy_primitives::bytes_::Bytes` is defined in crate `alloy_primitives`
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/alloy-primitives-0.7.0/src/bytes/mod.rs:21:1
    |
21  | pub struct Bytes(pub bytes::Bytes);
    | ^^^^^^^^^^^^^^^^
note: `alloy_primitives::Bytes` is defined in crate `alloy_primitives`
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/alloy-primitives-0.6.4/src/bytes/mod.rs:21:1
    |
21  | pub struct Bytes(pub bytes::Bytes);
    | ^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `alloy_primitives` are being used?

error[E0308]: mismatched types
   --> /Users/alexfertel/.cargo/git/checkouts/foundry-f7cca724e93059b0/dfab23e/crates/linking/src/lib.rs:187:74
    |
187 |                     bytecode.to_mut().link(file.to_string_lossy(), name, address);
    |                                       ----                               ^^^^^^^ expected `alloy_primitives::bits::address::Address`, found `Address`
    |                                       |
    |                                       arguments to this method are incorrect
    |
    = note: `Address` and `alloy_primitives::bits::address::Address` have similar names, but are actually distinct types
note: `Address` is defined in crate `alloy_primitives`
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/alloy-primitives-0.6.4/src/bits/address.rs:45:1
    |
45  | / wrap_fixed_bytes!(
46  | |     // we implement Display with the checksum, so we don't derive it
47  | |     extra_derives: [],
48  | |     /// An Ethereum address, 20 bytes in length.
...   |
86  | |     pub struct Address<20>;
87  | | );
    | |_^
note: `alloy_primitives::bits::address::Address` is defined in crate `alloy_primitives`
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/alloy-primitives-0.7.0/src/bits/address.rs:45:1
    |
45  | / wrap_fixed_bytes!(
46  | |     // we implement Display with the checksum, so we don't derive it
47  | |     extra_derives: [],
48  | |     /// An Ethereum address, 20 bytes in length.
...   |
86  | |     pub struct Address<20>;
87  | | );
    | |_^
    = note: perhaps two different versions of crate `alloy_primitives` are being used?
note: method defined here
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/foundry-compilers-0.3.14/src/artifacts/bytecode.rs:68:12
    |
68  |     pub fn link(
    |            ^^^^
    = note: this error originates in the macro `wrap_fixed_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
   --> /Users/alexfertel/.cargo/git/checkouts/foundry-f7cca724e93059b0/dfab23e/crates/linking/src/lib.rs:192:74
    |
192 |                     deployed_bytecode.link(file.to_string_lossy(), name, address);
    |                                       ----                               ^^^^^^^ expected `alloy_primitives::bits::address::Address`, found `Address`
    |                                       |
    |                                       arguments to this method are incorrect
    |
    = note: `Address` and `alloy_primitives::bits::address::Address` have similar names, but are actually distinct types
note: `Address` is defined in crate `alloy_primitives`
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/alloy-primitives-0.6.4/src/bits/address.rs:45:1
    |
45  | / wrap_fixed_bytes!(
46  | |     // we implement Display with the checksum, so we don't derive it
47  | |     extra_derives: [],
48  | |     /// An Ethereum address, 20 bytes in length.
...   |
86  | |     pub struct Address<20>;
87  | | );
    | |_^
note: `alloy_primitives::bits::address::Address` is defined in crate `alloy_primitives`
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/alloy-primitives-0.7.0/src/bits/address.rs:45:1
    |
45  | / wrap_fixed_bytes!(
46  | |     // we implement Display with the checksum, so we don't derive it
47  | |     extra_derives: [],
48  | |     /// An Ethereum address, 20 bytes in length.
...   |
86  | |     pub struct Address<20>;
87  | | );
    | |_^
    = note: perhaps two different versions of crate `alloy_primitives` are being used?
note: method defined here
   --> /Users/alexfertel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/foundry-compilers-0.3.14/src/artifacts/bytecode.rs:68:12
    |
68  |     pub fn link(
    |            ^^^^
    = note: this error originates in the macro `wrap_fixed_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0308`.
   Compiling unicode-segmentation v1.11.0
error: could not compile `foundry-linking` (lib) due to 3 previous errors
```
