use std::ffi::{c_char, CStr, CString};

const SOCKET_ADDR: &str = "127.0.0.1:54321";

fn main() {
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
        send_command_helper(SOCKET_ADDR, "info")
    );
}

fn send_command_helper(address: &str, command: &str) -> String {
    let command = CString::new(command).unwrap();
    let address = CString::new(address).unwrap();

    let result = unsafe {
        let lib =
            libloading::Library::new("smart_home_dyn_lib").expect("Failed to load smart library");
        let send_command = lib
            .get::<fn(*const c_char, *const c_char) -> *const c_char>(b"send_command")
            .expect("Failed to load send_command function");

        let result = send_command(address.as_ptr(), command.as_ptr());
        CStr::from_ptr(result)
    };

    String::from_utf8_lossy(result.to_bytes()).to_string()
}
