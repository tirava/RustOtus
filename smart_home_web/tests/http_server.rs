use actix_web::{http::StatusCode, test, web, web::Bytes, App};
use rand::Rng;
use smart_home_web::http_handler::prelude::*;
use smart_home_web::prelude::{AppData, DeviceStatus, SmartHouseError, SmartHouseStorageMemory};
use std::collections::HashMap;
use urlencoding::encode;

const HOUSE_NAME: &str = "Мой умный дом (http)";
const HOUSE_ADDRESS: &str = "ул. Умных домов, д.2, кв.3";
const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостиная";
const BEDROOM: &str = "Спальня";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SWITCH_1: &str = "Выключатель-1";
const SWITCH_2: &str = "Выключатель-2";

#[actix_web::test]
async fn test_rooms() {
    let expected = format!("[\"{LIVING_ROOM}\",\"{KITCHEN}\",\"{BEDROOM}\"]");

    test_helper_http("/rooms", expected).await;
}

#[actix_web::test]
async fn test_devices_in_rooms() {
    let kitchen = encode(KITCHEN).to_string();
    let path = "/devices/".to_owned() + &kitchen;
    let expected = format!("[\"{SWITCH_1}\",\"{SOCKET_1}\",\"{SOCKET_2}\"]");

    test_helper_http(&path, expected).await;
}

async fn test_helper_http(path: &str, expected: String) {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::clone(&data))
            .service(get_rooms)
            .service(get_room_devices),
    )
    .await;

    let req = test::TestRequest::get().uri(path).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let expected_json = expected;
    assert_eq!(body, Bytes::from(expected_json));
}

async fn new_house_http() -> Result<AppData, SmartHouseError> {
    let mut app_data = AppData::new(
        HOUSE_NAME.to_string(),
        HOUSE_ADDRESS.to_string(),
        Box::new(SmartHouseStorageMemory::new()),
    );
    app_data.storage.init(generate_mock_devices()).await?;

    Ok(app_data)
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
