mod database;
mod db;
mod model;
mod routes;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use log::info;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db_client = db::DbClient::new().await?;
    info!("Database client initialized.");

    let app_state = web::Data::new(model::state::AppState { db_client });

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
