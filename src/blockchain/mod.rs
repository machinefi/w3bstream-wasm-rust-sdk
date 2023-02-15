//! Interact with contracts on the blockchain in the wasm.
//!
//! ## Usage
//!
//! This module contains utility methods to write and read the contracts
//! on the blockchain.
//!
//! The functions provided are synchronous. The wasm will be blocked until
//! the result is returned from the blockchain.
mod jrpc;
pub use self::jrpc::*;
