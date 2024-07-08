use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::{env, io};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

pub struct HTTPServer {
    bind_address: String,
    workers: usize,
}

impl HTTPServer {
    pub fn new(bind_address: String, log_level: String, workers: usize) -> Self {
        env::set_var("RUST_LOG", log_level);
        env_logger::init();

        Self {
            bind_address,
            workers,
        }
    }

    pub async fn start(self) -> io::Result<()> {
        info!("Server is starting on: {} ...", self.bind_address);

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::new(
                    "%{r}a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T",
                ))
                .service(get_health_check)
                .service(get_smart_thermometer_info)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", ApiDoc::openapi()),
                )
                .service(web::redirect("/", "/swagger-ui/"))
        })
        .workers(self.workers)
        .bind(&self.bind_address)?
        .run()
        .await?;

        Ok(())
    }
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
pub(crate) struct ApiDoc;

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
