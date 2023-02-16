# w3bstream-wasm-rust-sdk

A Rust SDK for writing Webassembly for W3bstream.

[![Crates.io][crates-badge]][crates-url]
[![Apache licensed][apache-badge]][apache-url]

[crates-badge]: https://img.shields.io/crates/v/ws-sdk.svg
[crates-url]: https://crates.io/crates/ws-sdk
[apache-badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[apache-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

[Website](https://w3bstream.com/) |
[Guides](https://docs.w3bstream.com/) |
[API Docs](https://docs.rs/ws-sdk/latest/ws_sdk/)




## Example

Say “Hello World!” to the w3bstream with rust-sdk.

Make sure you added the ws-sdk crate on Cargo.toml:

```sh
cargo add ws-sdk
```
Then, on your lib.rs:

```rust
use ws_sdk::log::log_info;

#[no_mangle]
pub extern "C" fn start(_: i32) -> i32 {
    log_info("hello world!");
    return 0;
}
```

More examples can be found [here][examples]. 

[examples]: https://github.com/machinefi/w3bstream-wasm-rust-sdk/tree/main/examples
