use anyhow::Result;
use ws_sdk::database::kv::*;
use ws_sdk::log::log_info;

#[no_mangle]
pub extern "C" fn start(rid: i32) -> i32 {
    match handle(rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

fn handle(_: i32) -> Result<()> {
    set("key_test", String::from("test").into_bytes())?;
    log_info("set key success")?;

    let value = get("key_test")?;
    log_info("get key success")?;

    log_info(&format!(
        "get data {} by key {}",
        String::from_utf8(value)?,
        "key_test"
    ))?;

    Ok(())
}
