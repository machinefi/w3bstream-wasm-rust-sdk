use anyhow::Result;
use ws_sdk::log::log_info;
use ws_sdk::stream::*;

#[no_mangle]
pub extern "C" fn start(rid: i32) -> i32 {
    match handle(rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

fn handle(rid: i32) -> Result<()> {
    let key = String::from_utf8(get_data(rid as _)?)?;
    let env = get_env(&key)?;
    log_info(&format!("get env from host: [key:{}] [val:{}]", key, env,))?;
    Ok(())
}
