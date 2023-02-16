use std::mem;

// TODO: limit alloc size
#[no_mangle]
pub extern "C" fn alloc(size: i32) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(size as _);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr;
}
