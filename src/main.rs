#[path = "controller/controller.rs"]
mod controller;

mod app;
mod datasource;

use actix_web::{App, error, HttpServer, web};
use log::error;
use crate::app::app::AppState;
use crate::controller::crud_config::configure;
use crate::datasource::datasource::config::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool = create_pool(true).await?;

    HttpServer::new(move || {
        App::new()
            .configure(configure)
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, rq| {
                        error!("Error: {} at uri: {}", err.to_string(), rq.path());
                        error::ErrorBadRequest(err)
                    })
            )
            .app_data(web::Data::new(AppState {
                pool: pool.clone()
            }))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
