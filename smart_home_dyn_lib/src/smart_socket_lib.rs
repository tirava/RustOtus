use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn send_command(command: *const c_char) -> *const c_char {
    let command = String::from_utf8_lossy(CStr::from_ptr(command).to_bytes()).to_string();

    CString::new(format!("{} - OK", command))
        .unwrap()
        .into_raw()
}
