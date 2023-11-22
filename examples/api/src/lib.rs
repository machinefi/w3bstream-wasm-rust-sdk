use anyhow::Result;
use http::Request;
use ws_sdk::api::api_call;
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
    let json_str = r#"
    {
        "chainName":"iotex-testnet",
        "hash":"fcaf377ff3cc785d60c58de7e121d6a2e79e1c58c189ea8641f3ea61f7605285"
    }
    "#;

    let request = Request::builder()
        .method("GET")
        .uri("/system/read_tx")
        .header("EventType", "result")
        .body(json_str.as_bytes().to_vec())?;

    let ret = api_call(request)?;
    log_info(&format!("ret: {}", ret))?;
    Ok(())
}

#[no_mangle]
pub extern "C" fn handle_result(rid: i32) -> i32 {
    log_info(&format!("start rid: {}", rid));
    let data_str = String::from_utf8(get_data(rid as _).unwrap()).unwrap();
    log_info(&format!("get resource {}: `{}`", rid, data_str));
    0
}
