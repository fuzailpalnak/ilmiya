use crate::{database::queries, db, model};
use actix_web::{web, HttpResponse};
use anyhow::{Context, Result};

pub async fn delete(
    db_client: &db::DbClient,
    exam: &model::request::EditExamRequest,
) -> Result<HttpResponse> {
    queries::delete::delete_section_question_option_based_on_ids(&db_client.pool, &exam.delete)
        .await
        .context("Failed to delete sections/questions/options")?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn edit_exam(
    app_state: web::Data<model::state::AppState>,
    req_body: web::Json<model::request::EditExamRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    if !req_body.delete.is_all_empty() {
        delete(&app_state.db_client, &req_body).await.map_err(|e| {
            log::error!("Failed to update exam: {:?}", e);
            actix_web::error::ErrorInternalServerError("Internal server error")
        })?;
    }

    Ok(HttpResponse::Ok().finish())
}
