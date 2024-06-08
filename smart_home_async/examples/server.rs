use rand::Rng;
use smart_home_async::prelude::*;
use tokio::{select, time};

const SOCKET_ADDR: &str = "127.0.0.1:54321";
const THERMOMETER_ADDR: &str = "127.0.0.1:12345";

#[tokio::main]
async fn main() -> Result<(), SmartHouseError> {
    tokio::spawn(async move {
        for _ in 0..100 {
            time::sleep(time::Duration::from_secs_f32(1.5)).await;
            let temp = rand::thread_rng().gen_range(20.0..25.0);
            match SmartThermometer::send_command(THERMOMETER_ADDR, temp.to_string().as_str()).await
            {
                Ok(result) => {
                    println!("SERVER: SmartThermometer sensor sent datagram -> '{result}'")
                }
                Err(err) => eprintln!("SERVER: ThermSensor error -> {}", err),
            }
        }
    });

    let smart_thermometer =
        SmartThermometer::new("Термометрик".to_string(), "Комнатка-2".to_string(), 22.33)
            .listen(THERMOMETER_ADDR);

    let smart_socket = SmartSocket::new(
        "Розеточка".to_string(),
        "Комнатка-1".to_string(),
        DeviceStatus::On,
        111.222,
    )
    .listen(SOCKET_ADDR);

    match select! {
        st_result = smart_thermometer => st_result,
        ss_result = smart_socket => ss_result
    } {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("SERVER: error: {err}");
            Err(err)
        }
    }
}
