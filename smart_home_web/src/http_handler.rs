use actix_web::{get, head, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

pub mod prelude {
    pub use crate::http_handler::get_room_devices;
    pub use crate::http_handler::get_rooms;
    pub use crate::http_handler::head_health_check;
    pub use crate::http_handler::ApiDoc;
}

#[derive(OpenApi)]
#[openapi(
    paths(
        head_health_check,
        get_rooms,
        get_room_devices
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
    tag = "rooms",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[get("/rooms")]
async fn get_rooms() -> impl Responder {
    HttpResponse::Ok()
}

/// Get all devices in a room
#[utoipa::path(
    tag = "rooms",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[get("/rooms/{room}/devices")]
async fn get_room_devices() -> impl Responder {
    HttpResponse::Ok()
}
