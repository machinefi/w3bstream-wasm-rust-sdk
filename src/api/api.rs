use super::super::host::abi::*;
use anyhow::{bail, Result};
use base64::{engine::general_purpose, Engine as _};
use http::{Request, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct HttpRequest {
    Method: String,
    Url: String,
    Header: HashMap<String, Vec<String>>,
    Body: String,
}

#[derive(Deserialize)]
struct HttpResponse {
    Status: String,
    StatusCode: u32,
    Proto: String,
    Header: HashMap<String, Vec<String>>,
    Body: Vec<u8>,
}

/// call a system API
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::api::api_call;
/// use http::Request;
///
/// let request = Request::builder()
/// .method("GET")
/// .uri("/system/read_tx")
/// .header("eventType", "result")
/// .body(json_str.as_bytes().to_vec())?;
///
/// let ret = api_call(request)?;
/// ```
pub fn api_call(req: Request<Vec<u8>>) -> Result<String> {
    let header = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), vec![v.to_str().unwrap().to_string()]))
        .collect();
    let base64encoded = general_purpose::STANDARD.encode(req.body());
    let new_obj = HttpRequest {
        Method: req.method().to_string(),
        Url: String::from("w3bstream://w3bstream.com") + &req.uri().to_string(),
        Header: header,
        Body: base64encoded,
    };
    let obj_str = serde_json::to_string(&new_obj)?;

    // Ok(obj_str)

    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &mut (0 as i32) as *const i32;

    match unsafe { ws_api_call(obj_str.as_ptr(), obj_str.len() as _, data_ptr, data_size) } {
        0 => Ok(unsafe { String::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => bail!("fail to call the api"),
    }
}

pub fn decode_response(resp: &Vec<u8>) -> Result<HttpResponse> {
    let obj: HttpResponse = serde_json::from_slice(resp)?;
    Ok(obj)
}
