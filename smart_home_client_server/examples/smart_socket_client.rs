use smart_home_client_server::prelude::*;

const SOCKET_ADDR: &str = "127.0.0.1:54321";

fn main() -> Result<(), SmartHouseError> {
    let result = SmartSocket::send_command(SOCKET_ADDR, "info")?;
    println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "on")?;
    println!("CLIENT: SmartSocket command 'on' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "power")?;
    println!("CLIENT: SmartSocket command 'power' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "off")?;
    println!("CLIENT: SmartSocket command 'off' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "power")?;
    println!("CLIENT: SmartSocket command 'power' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "qqq")?;
    println!("CLIENT: SmartSocket command 'qqq' - '{}'\n", result);

    // let result = SmartThermometer::send_command("127.0.0.1:8282", "info")?;
    // println!("SmartThermometer command 'info': {:?}", result);

    // let result = SmartSwitch::send_command("127.0.0.1:8383", "info")?;
    // println!("SmartSwitch command 'info': {:?}", result);

    Ok(())
}
