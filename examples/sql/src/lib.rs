use anyhow::Result;
use ws_sdk::database::sql::*;
use ws_sdk::log::log_info;

#[no_mangle]
pub extern "C" fn start(rid: i32) -> i32 {
    match handle(rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

fn handle(rid: i32) -> Result<()> {
    execute("INSERT INTO table1 (ID) VALUES (?);", &[&rid])?;
    let ret = query("SELECT * FROM table1;", &[])?;
    log_info(&String::from_utf8(ret)?)?;
    Ok(())
}
