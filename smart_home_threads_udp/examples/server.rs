use smart_home_threads_udp::prelude::*;
use std::thread;

const SOCKET_ADDR: &str = "127.0.0.1:54321";
const THERMOMETER_ADDR: &str = "127.0.0.1:12345";

fn main() -> Result<(), SmartHouseError> {
    thread::spawn(move || {
        let mut smart_thermometer =
            SmartThermometer::new("Термометрик".to_string(), "Комнатка-2".to_string(), 22.33);
        match smart_thermometer.listen(THERMOMETER_ADDR) {
            Ok(_) => (),
            Err(err) => eprintln!("SMART_THERMOMETER: {}", err),
        }
    });

    let mut smart_socket = SmartSocket::new(
        "Розеточка".to_string(),
        "Комнатка-1".to_string(),
        DeviceStatus::On,
        111.222,
    );
    smart_socket.listen(SOCKET_ADDR)?;

    // let smart_switch = SmartSwitch::new(
    //     "Розеточка".to_string(),
    //     "Комнатка-3".to_string(),
    //     DeviceStatus::On,
    // );
    // smart_switch.listen("127.0.0.1:8383")?;

    Ok(())
}
