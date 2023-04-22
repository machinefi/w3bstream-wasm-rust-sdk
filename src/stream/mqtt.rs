use super::super::host::abi::*;
use anyhow::{bail, Result};

/// publish the topic with the payload via mqtt.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::stream::mqtt::publish
/// let payload = "test";
/// let res = publish("topic1", payload.as_bytes())?;
/// ```
pub fn publish(topic: &str, payload: &[u8]) -> Result<()> {
    match unsafe {
        ws_send_mqtt_msg(
            topic.as_ptr(),
            topic.len() as _,
            payload.as_ptr(),
            payload.len() as _,
        )
    } {
        0 => Ok(()),
        _ => bail!("fail to publish the topic via mqtt"),
    }
}
