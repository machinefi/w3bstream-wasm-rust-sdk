use anyhow::Result;
use ws_sdk::log::log_info;
use ws_sdk::stream::get_data;

#[no_mangle]
pub extern "C" fn start(rid: i32) -> i32 {
    match handle(rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

fn handle(rid: i32) -> Result<()> {
    log_info(&format!("start rid: {}", rid))?;
    let data_str = String::from_utf8(get_data(rid as _)?)?;
    log_info(&format!("get resource {}: `{}`", rid, data_str))?;
    Ok(())
}
