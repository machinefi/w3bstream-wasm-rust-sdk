use std::mem;

use anyhow::Result;
use ws_sdk::log::*;
use ws_sdk::stream::*;

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(size as _);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr;
}

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
