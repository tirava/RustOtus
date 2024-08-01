use crate::prelude::{AppData, SmartHouseError};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::{OpenApi, ToSchema};

pub mod prelude {
    pub use crate::http_handler::{
        delete_device, delete_room, get_device, get_house_report, get_room_devices, get_rooms,
        post_device, post_room,
    };
    pub use crate::http_handler::{ApiDoc, SmartDeviceInfo, SmartHouseReport};
}

const ROOM_NOT_FOUND: &str = "комната не найдена";
const DEVICE_NOT_FOUND: &str = "устройство не найдено";
const ROOM_OR_DEVICE_NOT_FOUND: &str = "комната или устройство не найдены";
const OK: &str = "OK";
const CONFLICT_ROOM_EXISTS: &str = "комната уже существует";
const CONFLICT_DEVICE_EXISTS: &str = "устройство уже существует";
const INTERNAL_SERVER_ERROR: &str = "внутренняя ошибка сервера";

#[derive(OpenApi)]
#[openapi(
    paths(
        get_rooms,
        post_room,
        delete_room,
        get_room_devices,
        post_device,
        delete_device,
        get_device,
        get_house_report
    ),
    components(
        schemas(SmartDeviceInfo, SmartHouseReport),
    ),
    tags(
        (name = "Smart Home REST API", description = "Умный дом с умными устройствами")
    ),
)]
pub struct ApiDoc;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct SmartDeviceInfo {
    pub(crate) name: String,
    pub(crate) status: String,
    pub(crate) power: f32,
    pub(crate) temp: f32,
}

impl SmartDeviceInfo {
    pub fn new(name: String, status: String, power: f32, temp: f32) -> Self {
        Self {
            name,
            status,
            power,
            temp,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct SmartHouseReport {
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) devices: BTreeMap<String, Vec<SmartDeviceInfo>>,
}

/// Список всех комнат
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = OK, body = [&str]),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[get("/rooms")]
async fn get_rooms(app_data: web::Data<AppData>) -> Result<impl Responder, SmartHouseError> {
    Ok(HttpResponse::Ok().json(app_data.rooms().await?))
}

/// Добавить комнату
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 201, description = OK),
        (status = 409, description = CONFLICT_ROOM_EXISTS),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[post("/room/{room_name}")]
async fn post_room(
    path: web::Path<String>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, SmartHouseError> {
    app_data.add_room(&path).await?;

    Ok(HttpResponse::Created())
}

/// Удалить комнату
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = OK),
        (status = 404, description = ROOM_NOT_FOUND),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[delete("/room/{room_name}")]
async fn delete_room(
    path: web::Path<String>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, SmartHouseError> {
    app_data.remove_room(&path).await?;

    Ok(HttpResponse::Ok())
}

/// Список всех устройств в комнате
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = [&str]),
        (status = 404, description = ROOM_NOT_FOUND),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[get("/devices/{room_name}")]
async fn get_room_devices(
    path: web::Path<String>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, SmartHouseError> {
    Ok(HttpResponse::Ok().json(app_data.devices(&path).await?))
}

/// Добавить устройство в комнату
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 201, description = OK),
        (status = 404, description = ROOM_NOT_FOUND),
        (status = 409, description = CONFLICT_DEVICE_EXISTS),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[post("/device/{device_name}/room/{room_name}")]
async fn post_device(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, SmartHouseError> {
    let (room_name, device_name) = path.into_inner();
    app_data.add_device(&device_name, &room_name).await?;

    Ok(HttpResponse::Created())
}

/// Удалить устройство из комнаты
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK),
        (status = 404, description = ROOM_OR_DEVICE_NOT_FOUND),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[delete("/device/{device_name}/room/{room_name}")]
async fn delete_device(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, SmartHouseError> {
    let (room_name, device_name) = path.into_inner();
    app_data.remove_device(&device_name, &room_name).await?;

    Ok(HttpResponse::Ok())
}

/// Статус устройства из источника информации
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = SmartDeviceInfo),
        (status = 404, description = ROOM_NOT_FOUND),
        (status = 409, description = DEVICE_NOT_FOUND),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[get("/device/{device_name}/room/{room_name}")]
async fn get_device(
    path: web::Path<(String, String)>,
    app_data: web::Data<AppData>,
) -> Result<impl Responder, SmartHouseError> {
    let (room_name, device_name) = path.into_inner();
    let device = app_data.device_info(&device_name, &room_name).await?;

    Ok(HttpResponse::Ok().json(device))
}

/// Отчёт о состоянии умного дома
#[utoipa::path(
    tag = "reports",
    responses(
        (status = 200, description = OK, body = SmartHouseReport),
        (status = 500, description = INTERNAL_SERVER_ERROR),
    )
)]
#[get("/house/report")]
async fn get_house_report(app_data: web::Data<AppData>) -> Result<impl Responder, SmartHouseError> {
    let house = app_data.house_report().await?;

    Ok(HttpResponse::Ok().json(house))
}

impl ResponseError for SmartHouseError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::RoomsNotFoundError => StatusCode::NOT_FOUND,
            Self::RoomNotFoundError(_) => StatusCode::NOT_FOUND,
            Self::RoomAlreadyExistsError(_) => StatusCode::CONFLICT,
            Self::DevicesNotFoundError => StatusCode::NOT_FOUND,
            Self::DeviceNotFoundError(_, _) => StatusCode::NOT_FOUND,
            Self::DeviceAlreadyExistsError(_, _) => StatusCode::CONFLICT,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ParseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DeviceInfoProviderError(_) => StatusCode::NOT_FOUND,
            Self::MongoDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IcedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::LibraryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::OtherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
