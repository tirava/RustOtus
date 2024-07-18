use actix_web::{http::Method, http::StatusCode, test, web, web::Bytes, App};
use smart_home_web::http_handler::prelude::*;
use smart_home_web::prelude::{AppData, DeviceStatus, SmartHouseError, SmartHouseStorageMemory};
use std::collections::HashMap;
use urlencoding::encode;

const HOUSE_NAME: &str = "Мой умный дом (http)";
const HOUSE_ADDRESS: &str = "ул. Умных домов, д.2, кв.3";
const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостиная";
const BEDROOM: &str = "Спальня";
const HALLWAY: &str = "Прихожая";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SWITCH_1: &str = "Выключатель-1";
const SWITCH_2: &str = "Выключатель-2";

#[actix_web::test]
async fn test_http_rooms() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);

    let expected = format!("[\"{LIVING_ROOM}\",\"{KITCHEN}\",\"{BEDROOM}\"]");
    test_http_helper(data, "/rooms", Method::GET, StatusCode::OK, expected).await;
}

#[actix_web::test]
async fn test_http_devices_in_rooms() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);
    let path = "/devices/".to_owned() + &encode(KITCHEN).to_string();

    let expected = format!("[\"{SWITCH_1}\",\"{SOCKET_1}\",\"{SOCKET_2}\"]");
    test_http_helper(data, &path, Method::GET, StatusCode::OK, expected).await;
}

#[actix_web::test]
async fn test_http_add_room() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);
    let path = "/room/".to_owned() + &encode(HALLWAY).to_string();
    test_http_helper(
        data.clone(),
        &path,
        Method::POST,
        StatusCode::CREATED,
        "".to_string(),
    )
    .await;

    let expected = format!("[\"{LIVING_ROOM}\",\"{KITCHEN}\",\"{HALLWAY}\",\"{BEDROOM}\"]");
    test_http_helper(data, "/rooms", Method::GET, StatusCode::OK, expected).await;
}

#[actix_web::test]
async fn test_http_remove_room() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);
    let path = "/room/".to_owned() + &encode(LIVING_ROOM).to_string();
    test_http_helper(
        data.clone(),
        &path,
        Method::DELETE,
        StatusCode::OK,
        "".to_string(),
    )
    .await;

    let expected = format!("[\"{KITCHEN}\",\"{BEDROOM}\"]");
    test_http_helper(data, "/rooms", Method::GET, StatusCode::OK, expected).await;
}

#[actix_web::test]
async fn test_http_room_device() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);
    let path = format!("/device/{}/room/{}", &encode(SOCKET_1), &encode(KITCHEN));

    let expected_json = serde_json::json!({
        "name": SOCKET_1,
        "status": DeviceStatus::On.to_string(),
        "power": 111.222,
        "temp": 0.0
    });
    test_http_helper(
        data,
        &path,
        Method::GET,
        StatusCode::OK,
        expected_json.to_string(),
    )
    .await;
}

#[actix_web::test]
async fn test_http_room_add_device() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);
    let path = format!("/device/{}/room/{}", &encode(SWITCH_2), &encode(KITCHEN));

    test_http_helper(
        data.clone(),
        &path,
        Method::POST,
        StatusCode::CREATED,
        "".to_string(),
    )
    .await;

    let path = "/devices/".to_owned() + &encode(KITCHEN).to_string();

    let expected = format!("[\"{SWITCH_1}\",\"{SWITCH_2}\",\"{SOCKET_1}\",\"{SOCKET_2}\"]");
    test_http_helper(data, &path, Method::GET, StatusCode::OK, expected).await;
}

#[actix_web::test]
async fn test_http_room_remove_device() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);
    let path = format!("/device/{}/room/{}", &encode(SWITCH_1), &encode(KITCHEN));

    test_http_helper(
        data.clone(),
        &path,
        Method::DELETE,
        StatusCode::OK,
        "".to_string(),
    )
    .await;

    let path = "/devices/".to_owned() + &encode(KITCHEN).to_string();
    let expected = format!("[\"{SOCKET_1}\",\"{SOCKET_2}\"]");

    test_http_helper(data, &path, Method::GET, StatusCode::OK, expected).await;
}

#[actix_web::test]
async fn test_http_house_report() {
    let app_data = new_house_http().await.unwrap();
    let data = web::Data::new(app_data);

    let expected_json = serde_json::json!({
      "name": HOUSE_NAME,
      "address": HOUSE_ADDRESS,
      "devices": {
        LIVING_ROOM: [
          {
            "name": SWITCH_2,
            "status": DeviceStatus::Off.to_string(),
            "power": 0.0,
            "temp": 0.0
          },
          {
            "name": SOCKET_1,
            "status": DeviceStatus::On.to_string(),
            "power": 222.333,
            "temp": 0.0
          },
          {
            "name": THERMOMETER_1,
            "status": DeviceStatus::Unknown.to_string(),
            "power": 0.0,
            "temp": 11.22
          }
        ],
        KITCHEN: [
          {
            "name": SWITCH_1,
            "status": DeviceStatus::On.to_string(),
            "power": 0.0,
            "temp": 0.0
          },
          {
            "name": SOCKET_1,
            "status": DeviceStatus::On.to_string(),
            "power": 111.222,
            "temp": 0.0
          },
          {
            "name": SOCKET_2,
            "status": DeviceStatus::Off.to_string(),
            "power": 0.0,
            "temp": 0.0
          }
        ],
        BEDROOM: [
          {
            "name": SWITCH_1,
            "status": DeviceStatus::Off.to_string(),
            "power": 0.0,
            "temp": 0.0
          },
          {
            "name": SWITCH_2,
            "status": DeviceStatus::On.to_string(),
            "power": 0.0,
            "temp": 0.0
          },
          {
            "name": THERMOMETER_2,
            "status": DeviceStatus::Unknown.to_string(),
            "power": 0.0,
            "temp": 22.33
          }
        ]
      }
    });
    test_http_helper(
        data,
        "/house/report",
        Method::GET,
        StatusCode::OK,
        expected_json.to_string(),
    )
    .await;
}

async fn test_http_helper(
    app_data: web::Data<AppData>,
    path: &str,
    method: Method,
    status_code: StatusCode,
    expected: String,
) {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::clone(&app_data))
            .service(get_rooms)
            .service(post_room)
            .service(delete_room)
            .service(get_room_devices)
            .service(post_device)
            .service(delete_device)
            .service(get_device)
            .service(get_house_report),
    )
    .await;

    let req = match method {
        Method::GET => test::TestRequest::get().uri(path).to_request(),
        Method::POST => test::TestRequest::post().uri(path).to_request(),
        Method::DELETE => test::TestRequest::delete().uri(path).to_request(),
        _ => unreachable!(),
    };
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), status_code);

    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::from(expected));
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
                        111.222,
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
                        11.22,
                    ),
                ),
                (
                    SOCKET_1,
                    SmartDeviceInfo::new(
                        SOCKET_1.to_string(),
                        DeviceStatus::On.to_string(),
                        222.333,
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
                        22.33,
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
