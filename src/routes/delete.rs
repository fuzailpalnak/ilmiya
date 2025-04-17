use crate::{db, models};

use crate::entities::exam;
use crate::errors::AppError;

use actix_web::{web, HttpResponse};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

/// Deletes an exam from the database by its ID.
///
/// # Arguments
///
/// * `db_client` - A reference to the database client
/// * `exam_id` - The ID of the exam to be deleted
///
/// # Example
/// ```no_run
/// use your_crate_name::{delete, db, errors::AppError};
///
/// #[tokio::main]
/// async fn main() -> Result<(), AppError> {
///     // Initialize a mock or real DB client here
///     let db_client = db::DbClient::new_test().await; // Example method
///     let exam_id = 1;
///
///     let result = delete(&db_client, exam_id).await;
///     assert!(result.is_ok());
///     Ok(())
/// }
/// ```
///
/// > **Note**: This test uses `no_run` because it depends on a database connection.
/// > For real tests, prefer using `#[cfg(test)]` with proper database setup.
async fn delete(db_client: &db::DbClient, exam_id: i32) -> Result<HttpResponse, AppError> {
    let db_conn = &db_client.db;

    exam::Entity::delete_many()
        .filter(exam::Column::Id.eq(exam_id))
        .exec(db_conn)
        .await
        .map_err(|e| AppError::DbErr(e))?;

    Ok(HttpResponse::Ok().finish())
}

/// Deletes an exam and all its associated data (sections, questions, options) by exam ID.
///
/// This endpoint is typically called via an HTTP DELETE request with the exam ID in the path.
/// The function parses the exam ID, validates it, and then performs the deletion through
/// a helper function that removes related data from the database.
///
/// # Arguments
/// * `app_state` - Shared application state containing the database client.
/// * `exam_id` - A path parameter representing the exam ID to be deleted (as a `String`).
///
/// # Returns
/// * `Ok(HttpResponse::Ok)` if the exam and its dependencies were deleted successfully.
/// * `Err(AppError)` if the exam ID is invalid or a database error occurs.
///
/// # Errors
/// * Returns `AppError::NotFound` if the exam ID is not a valid integer.
/// * Returns other variants of `AppError` if the deletion logic fails.
///
/// # Example
/// ```no_run
/// // Example route setup with Actix-Web
/// use actix_web::{web, HttpServer, App};
/// use my_app::handlers::delete_exam;
///
/// HttpServer::new(|| {
///     App::new()
///         .app_data(web::Data::new(AppState::new()))
///         .route("/exam/{id}", web::delete().to(delete_exam))
/// })
/// .bind("127.0.0.1:8080")?
/// .run()
/// .await?;
/// ```
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
