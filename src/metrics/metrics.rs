use super::super::host::abi::*;
use anyhow::{bail, Result};
use serde_json::Value;

/// submit a custom metircs
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::metrics::submit_metrics
/// let value = json!({
///     "temp": 90,
/// });
/// submit_metrics(value)?;
/// ```
pub fn submit_metrics(obj: Value) -> Result<()> {
    let obj_str = obj.to_string();
    match unsafe { ws_submit_metrics(obj_str.as_bytes().as_ptr(), obj_str.len() as _) } {
        0 => Ok(()),
        _ => bail!("fail to submit the metrics"),
    }
}
