use anyhow::Result;
use hex;
use ws_sdk::crypto::{self, *};
use ws_sdk::log::{self, log_info};

#[no_mangle]
pub extern "C" fn start(rid: i32) -> i32 {
    match handle(rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

static PVK_HEX: &str = "4582b2bf2611f8fe5f7d4e22e20ff19dda42ca630344b33831695c02b616c819";

fn handle(rid: i32) -> Result<()> {
    let message = rid.to_string();
    let pubkey_hex = crypto::secp256k1::pubkey(PVK_HEX)?;

    let sig_der = crypto::secp256k1::sign_der(PVK_HEX, message.as_bytes())?;
    assert!(crypto::secp256k1::verify_der(&pubkey_hex, message.as_bytes(), &sig_der).is_ok());

    let sig = crypto::secp256k1::sign(PVK_HEX, message.as_bytes()).unwrap();
    assert!(crypto::secp256k1::verify(&pubkey_hex, message.as_bytes(), &sig).is_ok());
    log_info("pass!");
    Ok(())
}
