use crate::prelude::{SmartDevice, SmartSocket};
use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn send_command(
    address: *const c_char,
    command: *const c_char,
) -> *const c_char {
    let command = String::from_utf8_lossy(CStr::from_ptr(command).to_bytes()).to_string();
    let address = String::from_utf8_lossy(CStr::from_ptr(address).to_bytes()).to_string();

    let result = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(SmartSocket::send_command(&address, &command))
        .unwrap_or_else(|err| err.to_string());

    CString::new(result).unwrap().into_raw()
}
