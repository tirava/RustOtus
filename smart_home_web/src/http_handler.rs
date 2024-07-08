use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

pub mod prelude {
    pub use crate::http_handler::get_health_check;
    pub use crate::http_handler::get_smart_thermometer_info;
    pub use crate::http_handler::ApiDoc;
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_health_check,
        get_smart_thermometer_info
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

#[utoipa::path(
    get,
    path = "/healthz",
    tag = "Health Check",
    responses(
        (status = 200, description = "OK", body = Response),
    )
)]
#[get("/healthz")]
async fn get_health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    get,
    tag = "Get Smart Thermometer Info",
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 500, description = "Internal Server Error", body = Response),
    )
)]
#[get("/smart-thermometer-info")]
async fn get_smart_thermometer_info() -> impl Responder {
    HttpResponse::Ok()
}
