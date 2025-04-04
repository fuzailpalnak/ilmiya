mod db;
mod endpoints;
mod entities;
mod errors;
mod models;
mod utils;

use crate::errors::AppError;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_client = db::DbClient::new().await?;
    let app_state = web::Data::new(models::AppState { db_client });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(endpoints::config_routes)
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await?;

    Ok(())
}
