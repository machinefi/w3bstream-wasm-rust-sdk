use anyhow::Result;
use serde_json::json;
use ws_sdk::log::log_info;
use ws_sdk::metrics::submit_metrics;

#[no_mangle]
pub extern "C" fn start(_rid: i32) -> i32 {
    match handle(_rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

fn handle(_rid: i32) -> Result<()> {
    let jsonified_data = json!({
        "name": "john",
        "age": 30,
    });
    submit_metrics(jsonified_data)?;
    log_info("metrics submitted")?;
    Ok(())
}
