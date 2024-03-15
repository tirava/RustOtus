use smart_home::*;

fn main() {
    let home = SmartHome::new("Умный дом".to_string(), "Адрес дома, кв.1".to_string());

    let kitchen = Room::new("Кухня".to_string(), &home);
    let bedroom = Room::new("Спальня".to_string(), &home);

    let thermometer = SmartThermometer::new(SmartDevice::new(
        "Умный термометр".to_string(),
        &kitchen,
        "127.0.0.1/api/thermometer".to_string(),
    ));
    thermometer.device.connect().unwrap_or_else(|err| {
        panic!(
            "Can't connect to {}, error: {err}",
            thermometer.device.name()
        )
    });

    println!(
        "Информация об устройстве '{}':\n{}",
        thermometer.device.name(),
        thermometer.device.info()
    );
    println!(
        "Температура с устройства '{}': {:.2} °С\n",
        thermometer.device.name(),
        thermometer.temperature().unwrap()
    );

    let mut socket = SmartPowerSocket::new(SmartDevice::new(
        "Умная розетка".to_string(),
        &bedroom,
        "127.0.0.1/api/socket".to_string(),
    ));
    socket
        .device
        .connect()
        .unwrap_or_else(|err| panic!("Can't connect to {}, error: {err}", socket.device.name()));

    println!(
        "Информация об устройстве '{}':\n{}",
        socket.device.name(),
        socket.device.info()
    );
    println!(
        "Состояние устройства '{}': {}",
        socket.device.name(),
        socket.get_state().unwrap()
    );
    println!(
        "Потребляемая мощность устройства '{}': {:.2} Вт\n",
        socket.device.name(),
        socket.power().unwrap()
    );

    socket.set_state(DeviceState::On).unwrap();
    println!(
        "Состояние устройства '{}': {}",
        socket.device.name(),
        socket.get_state().unwrap()
    );
    println!(
        "Потребляемая мощность устройства '{}': {:.2} Вт\n",
        socket.device.name(),
        socket.power().unwrap()
    );

    socket.set_state(DeviceState::Off).unwrap();
    println!(
        "Состояние устройства '{}': {}",
        socket.device.name(),
        socket.get_state().unwrap()
    );
    println!(
        "Потребляемая мощность устройства '{}': {:.2} Вт",
        socket.device.name(),
        socket.power().unwrap()
    );
}
