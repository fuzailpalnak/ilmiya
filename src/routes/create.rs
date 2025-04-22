use crate::{database, model};
use actix_web::{web, HttpResponse};
use anyhow::Result;

pub async fn create_exam(
    app_state: web::Data<model::state::AppState>,
    req_body: web::Json<model::request::ExamRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    database::queries::insert::insert_exam(&app_state.db_client.pool, &req_body)
        .await
        .map_err(|e| {
            log::error!("Failed to insert exam: {:?}", e);
            actix_web::error::ErrorInternalServerError("Internal server error")
        })?;

    Ok(HttpResponse::Created().json(req_body.exam_id.base.id))
}
