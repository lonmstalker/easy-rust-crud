mod lib;

use actix_web::{App, error, HttpServer, web};
use log::error;
use crate::lib::controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(controller::crud_config::configure)
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, rq| {
                        error!("Error: {} at uri: {}", err.to_string(), rq.path());
                        error::ErrorBadRequest(err)
                    })
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
