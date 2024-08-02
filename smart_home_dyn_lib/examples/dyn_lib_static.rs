use async_ffi::BorrowingFfiFuture;
use smart_home_dyn_lib::prelude::SmartHouseError;
use std::ffi::{c_char, CStr, CString};

const SOCKET_ADDR: &str = "127.0.0.1:54321";

extern "C" {
    fn send_command<'a>(
        address: *const c_char,
        command: *const c_char,
    ) -> BorrowingFfiFuture<'a, *mut c_char>;
}

#[tokio::main]
async fn main() -> Result<(), SmartHouseError> {
    // for listening TCP SmartSocket commands start server example before run dyn_lib

    // let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    // println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);
    //
    // let result = SmartSocket::send_command(SOCKET_ADDR, "on").await?;
    // println!("CLIENT: SmartSocket command 'on' - '{}'\n", result);
    // let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    // println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);
    //
    // let result = SmartSocket::send_command(SOCKET_ADDR, "off").await?;
    // println!("CLIENT: SmartSocket command 'off' - '{}'\n", result);
    // let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    // println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);
    //
    // let result = SmartSocket::send_command(SOCKET_ADDR, "qqq").await?;
    // println!("CLIENT: SmartSocket command 'qqq' - '{}'\n", result);

    println!(
        "CLIENT: SmartSocket command 'info' - '{}'",
        send_command_helper(SOCKET_ADDR, "info").await
    );

    Ok(())
}

async fn send_command_helper(address: &str, command: &str) -> String {
    let command = CString::new(command).unwrap();
    let address = CString::new(address).unwrap();

    let result = unsafe {
        let result = send_command(address.as_ptr(), command.as_ptr()).await;
        CStr::from_ptr(result)
    };

    String::from_utf8_lossy(result.to_bytes()).to_string()
}
