use super::super::host::abi::*;
use anyhow::{bail, Result};

/// Retrieves the payload in the event as the stream source.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::stream
/// let data = get_data(rid)?;
/// ```
pub fn get_data(resource_id: u32) -> Result<Vec<u8>> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &(0 as i32);
    match unsafe { ws_get_data(resource_id as _, data_ptr, data_size) } {
        0 => Ok(unsafe { Vec::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => bail!("fail to get data by resource id"),
    }
}

/// Sets the data for the sink of the stream.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::stream
/// set_data(rid, data)?;
/// ```
pub fn set_data(resource_id: u32, data: Vec<u8>) -> Result<()> {
    match unsafe { ws_set_data(resource_id as _, data.as_ptr(), data.len() as _) } {
        0 => Ok(()),
        _ => bail!("fail to set data by resource id"),
    }
}
