#[link(wasm_import_module = "env")]
extern "C" {
    pub fn ws_log(log_level: i32, ptr: *const u8, size: i32) -> i32;
    pub fn ws_get_data(
        resource_id: i32,
        return_ptr: *const *mut u8,
        return_size: *const i32,
    ) -> i32;
    pub fn ws_set_data(resource_id: i32, ptr: *const u8, size: i32) -> i32;
    pub fn ws_get_env(
        ptr: *const u8,
        size: i32,
        return_ptr: *const *mut u8,
        return_size: *const i32,
    ) -> i32;
    pub fn ws_get_db(
        key_ptr: *const u8,
        key_size: i32,
        return_ptr: *const *mut u8,
        return_size: *const i32,
    ) -> i32;
    pub fn ws_set_db(
        key_ptr: *const u8,
        key_size: i32,
        value_ptr: *const u8,
        value_size: i32,
    ) -> i32;
    pub fn ws_get_sql_db(
        ptr: *const u8,
        size: i32,
        return_ptr: *const *mut u8,
        return_size: *const i32,
    ) -> i32;
    pub fn ws_set_sql_db(ptr: *const u8, size: i32) -> i32;
    pub fn ws_send_tx(
        chain_id: i32,
        payload_ptr: *const u8,
        payload_size: i32,
        return_hash_ptr: *const *mut u8,
        return_hash_size: *const i32,
    ) -> i32;
    pub fn ws_call_contract(
        chain_id: i32,
        ptr: *const u8,
        size: i32,
        return_ptr: *const *mut u8,
        return_size: *const i32,
    ) -> i32;
}
