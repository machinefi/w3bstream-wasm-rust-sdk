//! Interact with database in the wasm.
//!
//! ## Usage
//!
//! This module contains utility methods to write and read the database.
//!
//! The functions provided are synchronous. The wasm will be blocked until
//! the result is returned from the database.
//!
//!

/// key-value database module.
pub mod kv;
/// SQL database module.
pub mod sql;
pub mod sql_types;
