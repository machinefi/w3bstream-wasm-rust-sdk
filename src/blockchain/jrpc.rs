use super::super::host::abi::*;
use anyhow::{bail, Result};
use serde::Serialize;

#[derive(Serialize)]
struct Tx {
    to: String,
    data: String,
    value: String,
}

/// Send a transaction to the blockchain whose chain id is `chain_id`.
///
/// The transaction is signed with the private key of the node operator.
/// The hash of transaction is returned as a string.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::blockchain
/// let hash = send_tx(4689, "0x83c9fb5da807e4427b59b6c90b545496394abf98", "0", "")?;
/// ```
pub fn send_tx(chain_id: u32, to: &str, value: &str, data: &str) -> Result<String> {
    let tx = Tx {
        to: to.to_owned(),
        value: value.to_owned(),
        data: data.to_owned(),
    };
    let str = serde_json::to_string(&tx)?;
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &mut (0 as i32) as *const i32;
    match unsafe {
        ws_send_tx(
            chain_id as _,
            str.as_ptr(),
            str.len() as _,
            data_ptr,
            data_size,
        )
    } {
        0 => Ok(unsafe { String::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => bail!("fail to send tx"),
    }
}

#[derive(Serialize)]
struct Call {
    to: String,
    data: String,
}

/// Call a contract on the blockchain whose chain id is `chain_id`.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::blockchain
/// let hex = call_contract(4689, "0x83c9fb5da807e4427b59b6c90b545496394abf98", "");
/// ```
pub fn call_contract(chain_id: u32, to: &str, data: &str) -> Result<Vec<u8>> {
    let tx = Call {
        to: to.to_owned(),
        data: data.to_owned(),
    };
    let str = serde_json::to_string(&tx)?;
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &mut (0 as i32) as *const i32;
    match unsafe {
        ws_call_contract(
            chain_id as _,
            str.as_ptr(),
            str.len() as _,
            data_ptr,
            data_size,
        )
    } {
        0 => Ok(unsafe { Vec::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => bail!("fail to call contract"),
    }
}
