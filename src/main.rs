pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod utils;
use crate::errors::AppError;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let redis_client = db::RedisClient::new().await?;
    let app_state = web::Data::new(models::AppState { redis_client });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(routes::config_routes)
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await?;

    Ok(())
}
