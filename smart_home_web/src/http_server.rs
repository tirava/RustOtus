use crate::app::AppData;
use crate::http_handler::prelude::*;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use log::info;
use parking_lot::Mutex;
use std::{env, io};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct HTTPServer {
    bind_address: String,
    workers: usize,
    app_data: AppData,
}

impl HTTPServer {
    pub fn new(bind_address: String, log_level: String, workers: usize, app_data: AppData) -> Self {
        env::set_var("RUST_LOG", log_level);
        env_logger::init();

        Self {
            bind_address,
            workers,
            app_data,
        }
    }

    pub async fn start(self) -> io::Result<()> {
        info!("Server is starting on: {} ...", self.bind_address);

        let data = web::Data::new(Mutex::new(self.app_data));

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::new(
                    "%{r}a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %D ms",
                ))
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", ApiDoc::openapi()),
                )
                .app_data(web::Data::clone(&data))
                .service(web::redirect("/", "/swagger-ui/"))
                .service(get_rooms)
                .service(get_room_devices)
                .service(post_room)
                .service(delete_room)
                .service(get_device)
                .service(post_device)
                .service(delete_device)
                .service(get_house_report)
        })
        .workers(self.workers)
        .bind(&self.bind_address)?
        .run()
        .await?;

        Ok(())
    }
}
