use crate::prelude::AppData;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

pub mod prelude {
    pub use crate::http_handler::ApiDoc;
    pub use crate::http_handler::{
        delete_device, delete_room, get_device, get_house_report, get_room_devices, get_rooms,
        post_device, post_room,
    };
}

const ERROR: &str = "Error";
const ROOM_NOT_FOUND: &str = "Room not found";
const OK: &str = "OK";
const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

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
        (name = "Smart Home REST API", description = "Smart Home with smart devices")
    ),
)]
pub struct ApiDoc;

#[derive(Serialize, Deserialize, ToSchema)]
struct Response {
    status: &'static str,
    message: &'static str,
}

/// Get all rooms
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = OK, body = [&str]),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[get("/rooms")]
async fn get_rooms(app_data: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().json(app_data.rooms())
}

/// Add room
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[post("/room/{room_name}")]
async fn post_room() -> impl Responder {
    HttpResponse::Ok()
}

/// Remove room
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[delete("/room/{room_name}")]
async fn delete_room() -> impl Responder {
    HttpResponse::Ok()
}

/// Get all devices from room
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = [&str]),
        (status = 404, description = ROOM_NOT_FOUND, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[get("/devices/{room_name}")]
async fn get_room_devices(path: web::Path<String>, app_data: web::Data<AppData>) -> impl Responder {
    let devices = match app_data.devices(path.as_str()) {
        Some(devices) => devices,
        None => {
            return HttpResponse::NotFound().json(Response {
                status: ERROR,
                message: ROOM_NOT_FOUND,
            })
        }
    };

    HttpResponse::Ok().json(devices)
}

/// Get device status
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[get("/device/{device_name}/room/{room_name}")]
async fn get_device() -> impl Responder {
    HttpResponse::Ok()
}

/// Add device into room
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = OK, body = Response),
        (status = 500, description = INTERNAL_SERVER_ERROR, body = Response),
    )
)]
#[post("/device/{device_name}/room/{room_name}")]
async fn post_device() -> impl Responder {
    HttpResponse::Ok()
}

/// Remove device from room
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

/// Get smart house report
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
