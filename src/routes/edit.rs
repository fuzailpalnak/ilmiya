use crate::{database::queries, conn, model};
use actix_web::{web, HttpResponse};
use anyhow::{Context, Result};

pub async fn delete(
    db_client: &conn::DbClient,
    exam: &model::exam::EditExamRequest,
) -> Result<HttpResponse> {
    queries::delete::delete_related_entities(&db_client.pool, &exam.delete)
        .await
        .context("Failed to delete sections/questions/options")?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn edit_exam(
    app_state: web::Data<model::state::AppState>,
    req_body: web::Json<model::exam::EditExamRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    if !req_body.delete.is_all_empty() {
        delete(&app_state.db_client, &req_body).await.map_err(|e| {
            log::error!("Failed to update exam: {:?}", e);
            actix_web::error::ErrorInternalServerError("Internal server error")
        })?;
    }

    Ok(HttpResponse::Ok().finish())
}
