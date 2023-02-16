use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use ws_sdk::log::log_info;

#[no_mangle]
pub extern "C" fn start(rid: i32) -> i32 {
    match handle(rid) {
        Ok(_) => return 0,
        _ => return -1,
    };
}

fn handle(_: i32) -> Result<()> {
    let json_str = r#"
    {
        "name":"John",
        "age":30,
        "car":null
    }
    "#;
    let obj: Value = serde_json::from_str(json_str)?;

    // get the value of the "name" key from the json
    let name = obj["name"].as_str().unwrap();
    log_info(&format!("name: {}", &name))?;

    // Create a Person struct and serialize it into a json string
    let person = Person {
        name: name.to_string(),
    };
    let json_bytes = serde_json::to_string(&person)?;
    log_info(&format!("encoded json: {}", json_bytes))?;
    Ok(())
}

#[derive(Serialize)]
struct Person {
    name: String,
}
