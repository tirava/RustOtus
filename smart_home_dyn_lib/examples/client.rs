use smart_home_dyn_lib::prelude::*;

const SOCKET_ADDR: &str = "127.0.0.1:54321";
const SWITCH_ADDR: &str = "127.0.0.1:31254";

#[tokio::main]
async fn main() -> Result<(), SmartHouseError> {
    let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "on").await?;
    println!("CLIENT: SmartSocket command 'on' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "power").await?;
    println!("CLIENT: SmartSocket command 'power' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "off").await?;
    println!("CLIENT: SmartSocket command 'off' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "power").await?;
    println!("CLIENT: SmartSocket command 'power' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "qqq").await?;
    println!("CLIENT: SmartSocket command 'qqq' - '{}'\n", result);

    let result = SmartSwitch::send_command(SWITCH_ADDR, "info").await?;
    println!("CLIENT: SmartSwitch command 'info' - '{}'\n", result);

    let result = SmartSwitch::send_command(SWITCH_ADDR, "on").await?;
    println!("CLIENT: SmartSwitch command 'on' - '{}'\n", result);
    let result = SmartSwitch::send_command(SWITCH_ADDR, "info").await?;
    println!("CLIENT: SmartSwitch command 'info' - '{}'\n", result);

    let result = SmartSwitch::send_command(SWITCH_ADDR, "off").await?;
    println!("CLIENT: SmartSwitch command 'off' - '{}'\n", result);
    let result = SmartSwitch::send_command(SWITCH_ADDR, "info").await?;
    println!("CLIENT: SmartSwitch command 'off' - '{}'\n", result);

    Ok(())
}
