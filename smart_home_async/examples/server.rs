use rand::Rng;
use smart_home_async::prelude::*;
use std::thread;

const SOCKET_ADDR: &str = "127.0.0.1:54321";
const THERMOMETER_ADDR: &str = "127.0.0.1:12345";

fn main() -> Result<(), SmartHouseError> {
    let mut smart_thermometer =
        SmartThermometer::new("Термометрик".to_string(), "Комнатка-2".to_string(), 22.33);
    thread::spawn(move || match smart_thermometer.listen(THERMOMETER_ADDR) {
        Ok(_) => (),
        Err(err) => eprintln!("SMART_THERMOMETER: {}", err),
    });

    thread::spawn(|| {
        for _ in 0..100 {
            thread::sleep(std::time::Duration::from_secs_f32(1.5));
            let temp = rand::thread_rng().gen_range(20.0..25.0);
            match SmartThermometer::send_command(THERMOMETER_ADDR, temp.to_string().as_str()) {
                Ok(result) => {
                    println!("SERVER: SmartThermometer sensor sent datagram -> '{result}'")
                }
                Err(err) => eprintln!("SERVER: ThermSensor error -> {}", err),
            }
        }
    });

    let mut smart_socket = SmartSocket::new(
        "Розеточка".to_string(),
        "Комнатка-1".to_string(),
        DeviceStatus::On,
        111.222,
    );
    smart_socket.listen(SOCKET_ADDR)?;

    Ok(())
}
