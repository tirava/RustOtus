use smart_home_dyn_lib::prelude::SmartHouseError;

const SOCKET_ADDR: &str = "127.0.0.1:54321";

extern "C" {
    fn get_integer() -> i32;
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

    let got = unsafe { get_integer() };
    println!("Got integer: {got}");

    Ok(())
}
