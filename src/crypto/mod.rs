//! Wrapped cryptography library (e.g. `secp256k1`, `secp256r1`) for data encryption and decryption
//!
//! ## Usage
//!
//! This module contains utility methods to do signature signing and verification.
//!
//! This module should be imported by enabling the `crypto` feature flag:
//! ```toml
//! ws-sdk = { version = ..., features = ["crypto"] }
//! ```
//! Besides, the `wasm32-unknown-unknown` target isn't supported with this module.
//! The `wasm32-wasi` target should be used when building the project
//! ```toml
//! cargo build --target=wasm32-wasi
//! ```
//!
//!
pub mod secp256k1;
pub mod secp256r1;
