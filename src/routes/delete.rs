use crate::{db, models};

use crate::entities::exam;
use crate::errors::AppError;

use actix_web::{web, HttpResponse};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

async fn delete(db_client: &db::DbClient, exam_id: i32) -> Result<HttpResponse, AppError> {
    let db_conn = &db_client.db;

    exam::Entity::delete_many()
        .filter(exam::Column::Id.eq(exam_id))
        .exec(db_conn)
        .await
        .map_err(|e| AppError::DbErr(e))?;

    Ok(HttpResponse::Ok().finish())
}

/// Handles the HTTP request to edit an exam.
pub async fn delete_exam(
    app_state: web::Data<models::AppState>,
    exam_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let exam_id_int: i32 = exam_id
        .into_inner()
        .parse()
        .map_err(|_| AppError::NotFound("Invalid exam ID".into()))?;

    delete(&app_state.db_client, exam_id_int).await?;
    Ok(HttpResponse::Ok().finish())
}
