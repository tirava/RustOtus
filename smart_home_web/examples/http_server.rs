use dashmap::DashMap;
use smart_home_web::prelude::*;
use std::env;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостиная";
const BEDROOM: &str = "Спальня";
const SOCKET: &str = "Розетка";
const THERMOMETER: &str = "Термометр";
const SWITCH: &str = "Выключатель";

#[actix_web::main]
async fn main() -> Result<(), SmartHouseError> {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    let bind_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8000".to_string());
    let workers = env::var("WORKERS").unwrap_or(2.to_string()).parse()?;

    let mut app_data = match env::var("MONGO_URI") {
        Ok(uri) => AppData::new(Box::new(
            SmartHouseStorageMongoDB::new(uri).connect().await?,
        )),
        Err(_) => AppData::new(Box::new(SmartHouseStorageMemory::new())),
    };

    app_data.storage.init(generate_mock_devices()).await?;

    HTTPServer::new(bind_address, log_level, workers, app_data)
        .start()
        .await?;

    Ok(())
}

fn generate_mock_devices() -> DashMap<String, DashMap<String, SmartDeviceInfo>> {
    let devices_info: DashMap<String, DashMap<String, SmartDeviceInfo>> = DashMap::new();

    devices_info.insert(KITCHEN.to_string(), DashMap::new());
    devices_info.insert(LIVING_ROOM.to_string(), DashMap::new());
    devices_info.insert(BEDROOM.to_string(), DashMap::new());

    devices_info.get_mut(KITCHEN).unwrap().insert(
        SOCKET.to_string(),
        SmartDeviceInfo::new(
            SOCKET.to_string(),
            DeviceStatus::On.to_string(),
            111.222,
            0.0,
        ),
    );

    devices_info.get_mut(LIVING_ROOM).unwrap().insert(
        THERMOMETER.to_string(),
        SmartDeviceInfo::new(
            THERMOMETER.to_string(),
            DeviceStatus::Unknown.to_string(),
            0.0,
            11.22,
        ),
    );

    devices_info.get_mut(BEDROOM).unwrap().insert(
        SWITCH.to_string(),
        SmartDeviceInfo::new(SWITCH.to_string(), DeviceStatus::Off.to_string(), 0.0, 0.0),
    );

    devices_info
}
