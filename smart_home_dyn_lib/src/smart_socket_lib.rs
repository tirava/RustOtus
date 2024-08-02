use crate::prelude::{SmartDevice, SmartSocket};
use async_ffi::{BorrowingFfiFuture, FutureExt};
use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn send_command<'a>(
    address: *const c_char,
    command: *const c_char,
) -> BorrowingFfiFuture<'a, *mut c_char> {
    let command = String::from_utf8_lossy(CStr::from_ptr(command).to_bytes()).to_string();
    let address = String::from_utf8_lossy(CStr::from_ptr(address).to_bytes()).to_string();

    async move {
        let result = SmartSocket::send_command(&address, &command)
            .await
            .unwrap_or_else(|err| err.to_string());
        CString::new(result).unwrap().into_raw()
    }
    .into_ffi()
}
