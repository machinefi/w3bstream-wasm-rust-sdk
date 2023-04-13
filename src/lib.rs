//! A Rust SDK for writing [Webassembly] for [W3bstream].
//!
//! [W3bstream] is a general framework for connecting data generated
//! in the physical world to the blockchain world. [Webassembly] is selected as the
//! programming language for data processing.
//!
//! At a high level, W3bstream provides several Application Binary Interfaces (ABIs)
//! to enhance the [Webassembly] ability for developers:
//!
//!  * [streaming]: Reading or Writing the data in the stream
//!  * [database]: Storing or accessing the data in the database
//!  * [blockchain]: Writing or reading the contract on the blockchain
//!  * [logging][log]: Logging information or errors
//!
//! Guide level documentation is found on the [website].
//!
//! [W3bstream]: https://w3bstream.com/
//! [Webassembly]: https://webassembly.org/
//! [streaming]: crate::stream
//! [database]: crate::database
//! [blockchain]: crate::blockchain
//! [log]: crate::log
//! [website]: https://docs.w3bstream.com/
//!
//! # Examples
//!
//! Say "Hello World!" to the w3bstream:
//!
//! ```no_run
//! use ws_sdk::log::log_info;
//!
//! #[no_mangle]
//! pub extern "C" fn start(_: i32) -> i32 {
//!     log_info("Hello World!");
//!     return 0;
//! }
//! ```
//!
pub mod blockchain;
pub mod crypto;
pub mod database;
mod host;
pub mod log;
pub mod stream;
