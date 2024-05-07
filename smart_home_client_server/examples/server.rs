use smart_home_client_server::prelude::*;

const SOCKET_ADDR: &str = "127.0.0.1:54321";

fn main() -> Result<(), SmartHouseError> {
    let mut smart_socket = SmartSocket::new(
        "Розеточка".to_string(),
        "Комнатка-1".to_string(),
        DeviceStatus::On,
        111.222,
    );
    smart_socket.listen(SOCKET_ADDR)?;

    // let smart_thermometer = SmartThermometer::new(
    //     "Термометрик".to_string(),
    //     "Комнатка-2".to_string(),
    //     22.33,
    // );
    // smart_thermometer.listen("127.0.0.1:8282")?;

    // let smart_switch = SmartSwitch::new(
    //     "Розеточка".to_string(),
    //     "Комнатка-3".to_string(),
    //     DeviceStatus::On,
    // );
    // smart_switch.listen("127.0.0.1:8383")?;

    Ok(())
}
