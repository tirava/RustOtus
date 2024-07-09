use actix_web::{delete, get, head, post, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use crate::prelude::AppData;

pub mod prelude {
    pub use crate::http_handler::ApiDoc;
    pub use crate::http_handler::{
        delete_device, delete_room, get_room_devices, get_rooms, get_rooms_report,
        head_health_check, post_device, post_room,
    };
}

#[derive(OpenApi)]
#[openapi(
    paths(
        head_health_check,
        get_rooms,
        get_room_devices,
        get_rooms_report,
        post_room,
        delete_room,
        post_device,
        delete_device
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
    message: String,
}

/// Health Checking
#[utoipa::path(
    tag = "Health Check",
    responses(
        (status = 200, description = "OK"),
    )
)]
#[head("/healthz")]
async fn head_health_check() -> impl Responder {
    HttpResponse::Ok()
}

/// Get all rooms
#[utoipa::path(
    tag = "rooms and devices",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[get("/rooms")]
async fn get_rooms(app_data: web::Data<AppData>,) -> impl Responder {
    HttpResponse::Ok()
}

/// Get all devices from room
#[utoipa::path(
    tag = "rooms and devices",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[get("/rooms/{room_name}/devices")]
async fn get_room_devices() -> impl Responder {
    HttpResponse::Ok()
}

/// Get smart house report
#[utoipa::path(
    tag = "rooms and devices",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[get("/rooms/report")]
async fn get_rooms_report() -> impl Responder {
    HttpResponse::Ok()
}

/// Add room
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
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
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[delete("/room/{room_name}")]
async fn delete_room() -> impl Responder {
    HttpResponse::Ok()
}

/// Add device into room
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[post("/room/{room_name}/device/{device_name}")]
async fn post_device() -> impl Responder {
    HttpResponse::Ok()
}

/// Remove device from room
#[utoipa::path(
    tag = "devices",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[delete("/room/{room_name}/device/{device_name}")]
async fn delete_device() -> impl Responder {
    HttpResponse::Ok()
}
