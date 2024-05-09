use rand::Rng;
use smart_home_threads_udp::prelude::*;
use std::thread::sleep;
use std::time;

const SOCKET_ADDR: &str = "127.0.0.1:54321";
const THERMOMETER_ADDR: &str = "127.0.0.1:12345";

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

    let result = SmartThermometer::send_command(THERMOMETER_ADDR, "info")?;
    println!("CLIENT: SmartThermometer command 'info' - '{}'\n", result);

    for _ in 0..10 {
        sleep(time::Duration::from_secs(1));
        let temp = rand::thread_rng().gen_range(20.0..25.0);
        let result = SmartThermometer::send_command(THERMOMETER_ADDR, temp.to_string().as_str())?;
        println!("CLIENT: SmartThermometer sent 'temp' - '{}'\n", result);
    }

    let result = SmartThermometer::send_command(THERMOMETER_ADDR, "info")?;
    println!("CLIENT: SmartThermometer command 'info' - '{}'\n", result);

    // let result = SmartThermometer::send_command("127.0.0.1:8282", "info")?;
    // println!("SmartThermometer command 'info': {:?}", result);

    // let result = SmartSwitch::send_command("127.0.0.1:8383", "info")?;
    // println!("SmartSwitch command 'info': {:?}", result);

    Ok(())
}
