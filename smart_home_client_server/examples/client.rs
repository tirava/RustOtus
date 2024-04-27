use smart_home_client_server::prelude::*;

fn main() -> Result<(), SmartHouseError> {
    let result = SmartSocket::send_command("127.0.0.1:8181", "info")?;
    println!("SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command("127.0.0.1:8181", "on")?;
    println!("SmartSocket command 'on' - '{}'\n", result);
    let result = SmartSocket::send_command("127.0.0.1:8181", "info")?;
    println!("SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command("127.0.0.1:8181", "off")?;
    println!("SmartSocket command 'off' - '{}'\n", result);
    let result = SmartSocket::send_command("127.0.0.1:8181", "info")?;
    println!("SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command("127.0.0.1:8181", "qqq")?;
    println!("SmartSocket command 'qqq' - '{}'\n", result);

    // let result = SmartThermometer::send_command("127.0.0.1:8282", "info")?;
    // println!("SmartThermometer command 'info': {:?}", result);

    // let result = SmartSwitch::send_command("127.0.0.1:8383", "info")?;
    // println!("SmartSwitch command 'info': {:?}", result);

    Ok(())
}
