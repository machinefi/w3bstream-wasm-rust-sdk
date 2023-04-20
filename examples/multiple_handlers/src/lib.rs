use ws_sdk::log::log_info;

#[no_mangle]
pub extern "C" fn log_hello(_: i32) -> i32 {
    match log_info("hello") {
        Ok(_) => return 0,
        _ => return -1,
    };
}

#[no_mangle]
pub extern "C" fn log_bye(_: i32) -> i32 {
    match log_info("bye") {
        Ok(_) => return 0,
        _ => return -1,
    };
}
