use super::super::host::abi::*;
use super::sql_types::*;
use anyhow::{bail, Result};

/// Execute a prepared statement with parameters.
///
/// This func is often used to write records into the database.
/// # Examples
///
/// ```no_run
/// use ws-sdk::database::sql
/// execute("INSERT INTO table (ID, COUNT) VALUES (?, ?);", &[&0, &32])?;
/// ```
pub fn execute(prepared: &str, params: &[&dyn SQLType]) -> Result<()> {
    let query = DBQuery {
        statement: prepared.to_string(),
        params: params.iter().map(|x| x.get_param()).collect(),
    };
    let encoded = &serde_json::to_string(&query)?;
    match unsafe { ws_set_sql_db(encoded.as_ptr(), encoded.len() as _) } {
        0 => Ok(()),
        _ => bail!("fail to exec sql"),
    }
}

/// Query a prepared statement with parameters.
///
/// This func is often used to read records from the database.
/// The query result is returned in `json` format.
/// # Examples
///
/// ```no_run
/// use ws-sdk::database::sql
/// let ret = query("SELECT * FROM table;", &[])?;
/// ```
pub fn query(prepared: &str, params: &[&dyn SQLType]) -> Result<Vec<u8>> {
    let query = DBQuery {
        statement: prepared.to_string(),
        params: params.iter().map(|x| x.get_param()).collect(),
    };
    let encoded = &serde_json::to_string(&query)?;
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &mut (0 as i32) as *const i32;

    match unsafe { ws_get_sql_db(encoded.as_ptr(), encoded.len() as _, data_ptr, data_size) } {
        0 => Ok(unsafe { Vec::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => bail!("fail to query sql"),
    }
}
