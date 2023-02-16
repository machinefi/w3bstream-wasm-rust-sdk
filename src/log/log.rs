use super::super::host::abi::*;
use anyhow::{bail, Result};

#[allow(dead_code)]
enum LogLevel {
    Trace = 1,
    Debug,
    Info,
    Warn,
    Error,
}

/// log an info message.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::log::log_info
/// log_info("hello world!")?;
/// ```
pub fn log_info(str: &str) -> Result<()> {
    match unsafe { ws_log(LogLevel::Info as _, str.as_bytes().as_ptr(), str.len() as _) } {
        0 => Ok(()),
        _ => bail!("fail to log the info"),
    }
}

/// log an error message.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::log::log_error
/// log_error("error!")?;
/// ```
pub fn log_error(str: &str) -> Result<()> {
    match unsafe {
        ws_log(
            LogLevel::Error as _,
            str.as_bytes().as_ptr(),
            str.len() as _,
        )
    } {
        0 => Ok(()),
        _ => bail!("fail to log the error"),
    }
}
