use crate::prelude::{AppData, SmartHouseError};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, web, HttpResponse, Responder, ResponseError};
use parking_lot::Mutex;
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

pub mod prelude {
    pub use crate::http_handler::ApiDoc;
    pub use crate::http_handler::{
        delete_device, delete_room, get_device, get_house_report, get_room_devices, get_rooms,
        post_device, post_room,
    };
}

const ROOM_NOT_FOUND: &str = "комната не найдена";
const ROOM_OR_DEVICE_NOT_FOUND: &str = "комната или устройство не найдены";
const OK: &str = "OK";
const CONFLICT_ROOM_EXISTS: &str = "комната уже существует";
const CONFLICT_DEVICE_EXISTS: &str = "устройство уже существует";
const INTERNAL_SERVER_ERROR: &str = "внутренняя ошибка сервера";

#[derive(OpenApi)]
#[openapi(
    paths(
        get_rooms,
        get_room_devices,
        post_room,
        delete_room,
        get_device,
        post_device,
        delete_device,
        get_house_report
    ),
    components(
        schemas(Response)
    ),
    tags(
        (name = "Smart Home REST API", description = "Умный дом с умными устройствами")
    ),
)]
pub struct ApiDoc;

#[derive(Serialize, ToSchema)]
struct Response {
    status: &'static str,
    message: String,
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
async fn get_rooms(app_data: web::Data<Mutex<AppData>>) -> Result<impl Responder, SmartHouseError> {
    let app_data = app_data.lock();
    let rooms = app_data.rooms().await?;

    Ok(HttpResponse::Ok().json(rooms))
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
    app_data: web::Data<Mutex<AppData>>,
) -> Result<impl Responder, SmartHouseError> {
    let mut app_data = app_data.lock();
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
    app_data: web::Data<Mutex<AppData>>,
) -> Result<impl Responder, SmartHouseError> {
    let mut app_data = app_data.lock();
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
    app_data: web::Data<Mutex<AppData>>,
) -> Result<impl Responder, SmartHouseError> {
    let app_data = app_data.lock();
    let devices = app_data.devices(&path).await?;

    Ok(HttpResponse::Ok().json(devices))
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
    app_data: web::Data<Mutex<AppData>>,
) -> Result<impl Responder, SmartHouseError> {
    let mut app_data = app_data.lock();
    let (room_name, device_name) = path.into_inner();
    app_data.add_device(&device_name, &room_name).await?;

    Ok(HttpResponse::Created())
}

/// Удалить устройство из комнаты
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 404, description = ROOM_OR_DEVICE_NOT_FOUND),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[delete("/device/{device_name}/room/{room_name}")]
async fn delete_device(
    path: web::Path<(String, String)>,
    app_data: web::Data<Mutex<AppData>>,
) -> Result<impl Responder, SmartHouseError> {
    let mut app_data = app_data.lock();
    let (room_name, device_name) = path.into_inner();
    app_data.remove_device(&device_name, &room_name).await?;

    Ok(HttpResponse::Ok())
}

/// Статус устройства
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[get("/device/{device_name}/room/{room_name}")]
async fn get_device() -> impl Responder {
    // todo
    HttpResponse::Ok()
}

/// Отчёт о состоянии умного дома
#[utoipa::path(
    tag = "reports",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[get("/house/report")]
async fn get_house_report() -> impl Responder {
    // todo
    HttpResponse::Ok()
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
        }
    }
}
