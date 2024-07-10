use crate::prelude::AppData;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

pub mod prelude {
    pub use crate::http_handler::ApiDoc;
    pub use crate::http_handler::{
        delete_device, delete_room, get_device, get_house_report, get_room_devices, get_rooms,
        post_device, post_room,
    };
}

const ERROR: &str = "ошибка";
const ROOM_NOT_FOUND: &str = "комната не найдена";
const OK: &str = "OK";
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

#[derive(Serialize, Deserialize, ToSchema)]
struct Response {
    status: &'static str,
    message: String,
}

/// Список всех комнат
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = OK, body = [&str]),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[get("/rooms")]
async fn get_rooms(app_data: web::Data<Mutex<AppData>>) -> impl Responder {
    let app_data = app_data.lock();

    HttpResponse::Ok().json(app_data.rooms())
}

/// Добавить комнату
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 201, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[post("/room/{room_name}")]
async fn post_room(path: web::Path<String>, app_data: web::Data<Mutex<AppData>>) -> impl Responder {
    let mut app_data = app_data.lock();
    if let Err(err) = app_data.add_room(path.as_str()) {
        return HttpResponse::InternalServerError().json(Response {
            status: ERROR,
            message: err.to_string(),
        });
    }

    HttpResponse::Created().into()
}

/// Удалить комнату
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[delete("/room/{room_name}")]
async fn delete_room(
    path: web::Path<String>,
    app_data: web::Data<Mutex<AppData>>,
) -> impl Responder {
    let mut app_data = app_data.lock();
    if let Err(err) = app_data.remove_room(path.as_str()) {
        return HttpResponse::InternalServerError().json(Response {
            status: ERROR,
            message: err.to_string(),
        });
    }

    HttpResponse::Ok().into()
}

/// Список всех устройств в комнате
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = [&str]),
        (status = 404, description = ROOM_NOT_FOUND, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[get("/devices/{room_name}")]
async fn get_room_devices(
    path: web::Path<String>,
    app_data: web::Data<Mutex<AppData>>,
) -> impl Responder {
    let app_data = app_data.lock();
    let devices = match app_data.devices(path.as_str()) {
        Some(devices) => devices,
        None => {
            return HttpResponse::NotFound().json(Response {
                status: ERROR,
                message: ROOM_NOT_FOUND.to_string(),
            })
        }
    };

    HttpResponse::Ok().json(devices)
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

/// Добавить устройство в комнату
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 201, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[post("/device/{device_name}/room/{room_name}")]
async fn post_device() -> impl Responder {
    HttpResponse::Created()
}

/// Удалить устройство из комнаты
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[delete("/device/{device_name}/room/{room_name}")]
async fn delete_device() -> impl Responder {
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
