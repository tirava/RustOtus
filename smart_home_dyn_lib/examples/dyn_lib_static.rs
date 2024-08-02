use smart_home_dyn_lib::prelude::SmartHouseError;
use std::ffi::{c_char, CStr, CString};

// const SOCKET_ADDR: &str = "127.0.0.1:54321";

extern "C" {
    fn send_command(command: *const c_char) -> *const c_char;
}

fn main() -> Result<(), SmartHouseError> {
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
        send_command_helper("info")
    );

    Ok(())
}

fn send_command_helper(command: &str) -> String {
    let command = CString::new(command).unwrap();
    let result = unsafe {
        let result = send_command(command.as_ptr());
        CStr::from_ptr(result)
    };

    String::from_utf8_lossy(result.to_bytes()).to_string()
}
