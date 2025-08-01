mod database;
mod conn;
mod model;
mod routes;
mod utils;
mod services;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use log::info;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();    

    let db_client = conn::DbClient::new().await?;
    db_client.run_migrations().await?;
    info!("Database client initialized.");

    let redis_client = conn::RedisClient::new().await?;

    let app_state = web::Data::new(model::state::AppState { db_client, redis_client });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8081")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .supports_credentials(),
            )
            .configure(routes::config_routes)
    })
    .bind("0.0.0.0:8000")?
    .workers(4)
    .run()
    .await?;

    Ok(())
}
