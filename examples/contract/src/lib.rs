use anyhow::Result;
use hex;
use ws_sdk::blockchain::*;
use ws_sdk::log::log_info;

mod contract;

const CONTRACT_ADDR: &str = "0x3908c0620ABC34a23A078097e2e878AFf60bbC28";

#[no_mangle]
pub extern "C" fn start(rid: i32) -> i32 {
    match handle(rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

fn handle(rid: i32) -> Result<()> {
    // get current number on the contract
    let encoded_get = contract::SET_GET.function("get")?.encode_input(&[])?;
    let data_get = hex::encode(encoded_get);
    let ret = call_contract(4690, CONTRACT_ADDR, &data_get)?;
    log_info(&format!("get current num: {:?}", ret))?;

    // set the number on the contract to `rid % 100`
    let new_num = rid % 100;
    let encoded_set = contract::SET_GET
        .function("set")?
        .encode_input(&[ethabi::Token::Uint(ethabi::Uint::from(new_num))])?;
    let data_set = hex::encode(encoded_set);
    let hash = send_tx(4690, CONTRACT_ADDR, "0", &data_set)?;

    log_info(&format!("set new num: {}, tx hash: {}", new_num, hash))?;

    Ok(())
}
