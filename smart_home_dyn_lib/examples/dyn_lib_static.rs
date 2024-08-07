use smart_home_dyn_lib::prelude::SmartHouseError;
use std::ffi::{c_char, CStr, CString};

const SOCKET_ADDR: &str = "127.0.0.1:54321";

extern "C" {
    fn send_command(address: *const c_char, command: *const c_char) -> *const c_char;
}

fn main() -> Result<(), SmartHouseError> {
    // for listening TCP SmartSocket commands start server example before run dyn_lib

    println!(
        "CLIENT: SmartSocket command 'info' - '{}'\n",
        send_command_helper(SOCKET_ADDR, "info")
    );

    println!(
        "CLIENT: SmartSocket command 'on' - '{}'",
        send_command_helper(SOCKET_ADDR, "on")
    );
    println!(
        "CLIENT: SmartSocket command 'info' - '{}'\n",
        send_command_helper(SOCKET_ADDR, "info")
    );

    println!(
        "CLIENT: SmartSocket command 'off' - '{}'",
        send_command_helper(SOCKET_ADDR, "off")
    );
    println!(
        "CLIENT: SmartSocket command 'info' - '{}'\n",
        send_command_helper(SOCKET_ADDR, "info")
    );

    println!(
        "CLIENT: SmartSocket command 'qqq' - '{}'",
        send_command_helper(SOCKET_ADDR, "qqq")
    );

    Ok(())
}

fn send_command_helper(address: &str, command: &str) -> String {
    let command = CString::new(command).unwrap();
    let address = CString::new(address).unwrap();

    let result = unsafe {
        let result = send_command(address.as_ptr(), command.as_ptr());
        CStr::from_ptr(result)
    };

    String::from_utf8_lossy(result.to_bytes()).to_string()
}
