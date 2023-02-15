use super::super::host::abi::*;
use anyhow::{bail, Result};

/// Retrieves the value for a key from the key-value database.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::database::kv
/// let value = get(key)?;
/// ```
pub fn get(key: &str) -> Result<Vec<u8>> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &(0 as i32);
    match unsafe { ws_get_db(key.as_bytes().as_ptr(), key.len() as _, data_ptr, data_size) } {
        0 => Ok(unsafe { Vec::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => bail!("fail to get the value by the key"),
    }
}

/// Sets the value for a key in the key-value database.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::database::kv
/// set("key", vec![])?;
/// ```
pub fn set(key: &str, value: Vec<u8>) -> Result<()> {
    match unsafe {
        ws_set_db(
            key.as_bytes().as_ptr(),
            key.len() as _,
            value.as_ptr(),
            value.len() as _,
        )
    } {
        0 => Ok(()),
        _ => bail!("fail to set the value by the key"),
    }
}
