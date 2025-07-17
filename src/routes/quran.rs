use crate::model::{
    self,
    quran::{QuranApiRedisResponse, QuranApiRequest},
};
use actix_web::{web, HttpResponse};
use anyhow::Result;
use deadpool_redis::redis::AsyncCommands;
use serde_json::from_str;

pub async fn get_quran_verse_indo_pak_script(
    app_state: web::Data<model::state::AppState>,
    req_body: web::Json<QuranApiRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let key = format!("quran:{}:{}", req_body.surah, req_body.verse);

    let mut conn = app_state.redis_client.get_connection().await.map_err(|e| {
        log::error!("Redis connection error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to connect to Redis")
    })?;

    let raw_value: Option<String> = conn.get(&key).await.map_err(|e| {
        log::error!("Redis GET error for key {}: {:?}", key, e);
        actix_web::error::ErrorInternalServerError("Failed to retrieve verse from Redis")
    })?;

    match raw_value {
        Some(json_str) => {
            let words: Vec<String> = from_str(&json_str).map_err(|e| {
                log::error!("Failed to parse Redis JSON string: {:?}", e);
                actix_web::error::ErrorInternalServerError("Failed to parse verse data")
            })?;

            let joined = words.join(", ");

            let response = QuranApiRedisResponse {
                text: vec![joined],
                mode: "indopak".to_string(), // Change this if needed
            };

            Ok(HttpResponse::Ok().json(response))
        }
        None => Ok(HttpResponse::NotFound().body("Verse not found")),
    }
}
