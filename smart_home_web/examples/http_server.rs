use rand::Rng;
use smart_home_web::prelude::*;
use std::collections::HashMap;
use std::env;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостиная";
const BEDROOM: &str = "Спальня";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SWITCH_1: &str = "Выключатель-1";
const SWITCH_2: &str = "Выключатель-2";

#[actix_web::main]
async fn main() -> Result<(), SmartHouseError> {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    env_logger::init();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8000".to_string());
    let workers = env::var("WORKERS").unwrap_or(2.to_string()).parse()?;

    let house_name = "Мой умный дом".to_string();
    let house_address = "ул. Умных домов, д.1, кв.2".to_string();

    // MONGO_URI=mongodb://user:password@localhost:27017/my_db
    let mut app_data = match env::var("MONGO_DB_URI") {
        Ok(uri) => AppData::new(
            format!("{house_name} (MongoDB)"),
            house_address,
            Box::new(SmartHouseStorageMongoDB::new(&uri).await?),
        ),
        Err(_) => AppData::new(
            house_name,
            house_address,
            Box::new(SmartHouseStorageMemory::new()),
        ),
    };

    app_data.storage.init(generate_mock_devices()).await?;

    HTTPServer::new(bind_address, workers, app_data)
        .start()
        .await?;

    Ok(())
}

fn generate_mock_devices() -> HashMap<&'static str, HashMap<&'static str, SmartDeviceInfo>> {
    let devices_info = HashMap::from([
        (
            KITCHEN,
            HashMap::from([
                (
                    SOCKET_1,
                    SmartDeviceInfo::new(
                        SOCKET_1.to_string(),
                        DeviceStatus::On.to_string(),
                        rand::thread_rng().gen_range(10.0..3000.0),
                        0.0,
                    ),
                ),
                (
                    SOCKET_2,
                    SmartDeviceInfo::new(
                        SOCKET_2.to_string(),
                        DeviceStatus::Off.to_string(),
                        0.0,
                        0.0,
                    ),
                ),
                (
                    SWITCH_1,
                    SmartDeviceInfo::new(
                        SWITCH_1.to_string(),
                        DeviceStatus::On.to_string(),
                        0.0,
                        0.0,
                    ),
                ),
            ]),
        ),
        (
            LIVING_ROOM,
            HashMap::from([
                (
                    THERMOMETER_1,
                    SmartDeviceInfo::new(
                        THERMOMETER_1.to_string(),
                        DeviceStatus::Unknown.to_string(),
                        0.0,
                        rand::thread_rng().gen_range(20.0..25.0),
                    ),
                ),
                (
                    SOCKET_1,
                    SmartDeviceInfo::new(
                        SOCKET_1.to_string(),
                        DeviceStatus::On.to_string(),
                        rand::thread_rng().gen_range(100.0..300.0),
                        0.0,
                    ),
                ),
                (
                    SWITCH_2,
                    SmartDeviceInfo::new(
                        SWITCH_2.to_string(),
                        DeviceStatus::Off.to_string(),
                        0.0,
                        0.0,
                    ),
                ),
            ]),
        ),
        (
            BEDROOM,
            HashMap::from([
                (
                    THERMOMETER_2,
                    SmartDeviceInfo::new(
                        THERMOMETER_2.to_string(),
                        DeviceStatus::Unknown.to_string(),
                        0.0,
                        rand::thread_rng().gen_range(18.0..20.0),
                    ),
                ),
                (
                    SWITCH_1,
                    SmartDeviceInfo::new(
                        SWITCH_1.to_string(),
                        DeviceStatus::Off.to_string(),
                        0.0,
                        0.0,
                    ),
                ),
                (
                    SWITCH_2,
                    SmartDeviceInfo::new(
                        SWITCH_2.to_string(),
                        DeviceStatus::On.to_string(),
                        0.0,
                        0.0,
                    ),
                ),
            ]),
        ),
    ]);

    devices_info
}
