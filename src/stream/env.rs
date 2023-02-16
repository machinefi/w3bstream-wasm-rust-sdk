use super::super::host::abi::*;
use anyhow::{bail, Result};

/// Retrieves the environment variables of the project by the key.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::stream
/// let env = get_env("key")?;
/// ```
pub fn get_env(key: &str) -> Result<String> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &mut (0 as i32) as *const i32;
    match unsafe { ws_get_env(key.as_bytes().as_ptr(), key.len() as _, data_ptr, data_size) } {
        0 => Ok(unsafe { String::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => bail!("fail to get env"),
    }
}
