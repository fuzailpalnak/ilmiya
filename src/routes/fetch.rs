use crate::database::queries;
use crate::model;
use actix_web::{web, HttpResponse};
use anyhow::Result;

pub async fn fetch_exam(
    app_state: web::Data<model::state::AppState>,
    exam_id: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let exam_id_int: i32 = exam_id.into_inner().parse().map_err(|e| {
        log::error!("Failed to fetch exam: {:?}", e);
        actix_web::error::ErrorInternalServerError("Internal server error")
    })?;

    let exam_data = queries::read::read_exam_data(&app_state.db_client.pool, exam_id_int)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch exam: {:?}", e);
            actix_web::error::ErrorInternalServerError("Internal server error")
        })?;

    Ok(HttpResponse::Ok().json(exam_data))
}
