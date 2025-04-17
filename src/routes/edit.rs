use crate::models::{OptionModel, Question, Section};
use crate::{db, models};

use crate::entities::{details, exam, options, questions, sections};
use crate::errors::AppError;

use actix_web::{web, HttpResponse};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::JoinType;
use sea_orm::QuerySelect;
use sea_orm::RelationTrait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

/// Runs the query to retrieve the exam, sections, questions, and options by their IDs.
/// This function interacts with the database and returns the raw data.
///
/// # Arguments
/// * `db_client` - A reference to the database client to interact with the database.
/// * `exam_id` - The ID of the exam to be fetched.
///
/// # Returns
/// * `Result<Vec<(Option<i32>, Option<i32>, Option<i32>)>, AppError>` - A result containing the query response data or an error if the operation fails.
///
/// # Example
/// ```rust
/// let result = fetch_exam_ids(&db_client, 1).await;
/// match result {
///     Ok(response) => println!("Fetched exam ids: {:?}", response),
///     Err(e) => println!("Error fetching exam ids: {}", e),
/// }
/// ```
async fn fetch_ids_query(
    db_client: &db::DbClient,
    exam_id: i32,
) -> Result<Vec<(Option<i32>, Option<i32>, Option<i32>)>, AppError> {
    let db_conn = &db_client.db; // Directly borrow db reference

    let select = exam::Entity::find()
        .filter(exam::Column::Id.eq(exam_id))
        .select_only(); // Select only the specified columns

    let result = db::Prefixer::new(select.clone())
        .add_columns_from_list(sections::Entity, &[sections::Column::Id])
        .add_columns_from_list(questions::Entity, &[questions::Column::Id])
        .add_columns_from_list(options::Entity, &[options::Column::Id])
        .selector
        .join(JoinType::LeftJoin, exam::Relation::Details.def())
        .join(JoinType::LeftJoin, details::Relation::Sections.def())
        .join(JoinType::LeftJoin, sections::Relation::Questions.def())
        .join(JoinType::LeftJoin, questions::Relation::Options.def())
        .into_tuple::<(Option<i32>, Option<i32>, Option<i32>)>()
        .all(db_conn)
        .await?;

    Ok(result)
}

/// Converts a vector of optional integer tuples into a vector of non-optional tuples,
/// replacing any `None` values with `0`.
///
/// This is typically used to sanitize database query results where some fields may be `NULL`,
/// ensuring the resulting data is fully populated with default values.
///
/// # Arguments
/// * `input` - A vector of tuples, each containing three optional `i32` values.
///
/// # Returns
/// * A vector of tuples where all `None` values are replaced by `0`.
///
/// # Example
/// ```
/// let input = vec![
///     (Some(1), Some(2), Some(3)),
///     (Some(4), None, Some(6)),
///     (None, None, None),
/// ];
/// let output = generate_final_response(input);
/// assert_eq!(output, vec![(1, 2, 3), (4, 0, 6), (0, 0, 0)]);
/// ```
fn generate_final_response(
    input: Vec<(Option<i32>, Option<i32>, Option<i32>)>,
) -> Vec<(i32, i32, i32)> {
    input
        .into_iter()
        .map(|(a, b, c)| (a.unwrap_or(0), b.unwrap_or(0), c.unwrap_or(0)))
        .collect()
}

/// Deletes sections, questions, and options from the database based on the provided `DeletionData`.
///
/// This function handles deletions in the following order:
/// 1. If `section_id_to_delete` is not empty, it deletes the matching sections. If foreign keys are set with
///    cascading delete, related questions and options will also be deleted automatically.
/// 2. If `question_id_to_delete` is not empty, it deletes the corresponding questions.
/// 3. If `option_id_to_delete` is not empty, it deletes the corresponding options.
///
/// Before executing the deletions, `deletion_data.filter_deletions()` is called to ensure that IDs
/// that should not be deleted (e.g. due to conflicts or dependencies) are filtered out.
///
/// # Arguments
/// * `db_client` - A reference to the database client containing the DB connection.
/// * `deletion_data` - A mutable `DeletionData` struct that includes lists of IDs to be deleted.
///
/// # Returns
/// * `Ok(())` if all deletions were successful.
/// * `Err(AppError)` if a database error occurred.
///
/// # Example
/// ```no_run
/// use crate::models::DeletionData;
/// use crate::db::DbClient;
/// use crate::errors::AppError;
///
/// let mut deletion_data = DeletionData::new(
///     vec![1, 2],     // section_ids
///     vec![10, 11],   // question_ids
///     vec![100, 101], // option_ids
///     vec![],         // filtered or irrelevant data
/// );
///
/// let db_client = DbClient::new(); // Assume this gives a valid db client
///
/// actix_rt::spawn(async move {
///     match delete_query(&db_client, deletion_data).await {
///         Ok(_) => println!("Delete successful"),
///         Err(e) => eprintln!("Delete failed: {:?}", e),
///     }
/// });
/// ```
async fn delete_query(
    db_client: &db::DbClient,
    mut deletion_data: models::DeletionData,
) -> Result<(), AppError> {
    let db_conn = &db_client.db;

    // Clean the deletion data before executing queries
    deletion_data.filter_deletions();

    // If section_ids are provided, let cascading foreign key constraints handle dependent deletions
    if !deletion_data.section_id_to_delete.is_empty() {
        sections::Entity::delete_many()
            .filter(
                Expr::col(sections::Column::Id).is_in(deletion_data.section_id_to_delete.clone()),
            )
            .exec(db_conn)
            .await
            .map_err(AppError::DbErr)?;
    }

    // Delete questions directly if not already deleted via section cascade
    if !deletion_data.question_id_to_delete.is_empty() {
        questions::Entity::delete_many()
            .filter(
                Expr::col(questions::Column::Id).is_in(deletion_data.question_id_to_delete.clone()),
            )
            .exec(db_conn)
            .await
            .map_err(AppError::DbErr)?;
    }

    // Delete standalone options not already removed through question/section deletion
    if !deletion_data.option_id_to_delete.is_empty() {
        options::Entity::delete_many()
            .filter(Expr::col(options::Column::Id).is_in(deletion_data.option_id_to_delete.clone()))
            .exec(db_conn)
            .await
            .map_err(AppError::DbErr)?;
    }

    Ok(())
}

/// Deletes the exam with the given `exam_id` and associated sections, questions, and options.
/// It fetches the relevant IDs from the database, prepares deletion data, and performs the deletion.
///
/// # Arguments
/// * `db_client` - A reference to the database client used to perform database operations.
/// * `exam` - The `EditExam` struct containing the information of the exam to be deleted, including the IDs of the sections, questions, and options to delete.
///
/// # Returns
/// * `Result<HttpResponse, AppError>` - A `Result` containing either an `HttpResponse` (OK) on success, or an `AppError` if something goes wrong.
///
/// # Example
/// ```rust
/// use crate::models::{EditExam, DeletionData};
/// use crate::db::DbClient;
/// use actix_web::HttpResponse;
///
/// let exam_id = 1;
/// let delete_exam_data = EditExam {
///     exam_id: exam_id.into(),
///     delete: DeletionData {
///         section_ids: vec![1, 2],
///         question_ids: vec![3, 4],
///         option_ids: vec![5, 6],
///     },
///     ..Default::default() // Default values for other fields
/// };
///
/// let db_client = DbClient::new(); // Assuming you have a way to instantiate this
/// let result = delete(&db_client, &delete_exam_data).await;
///
/// match result {
///     Ok(response) => {
///         assert_eq!(response.status(), 200); // Ensure a 200 OK response
///     },
///     Err(e) => panic!("Error occurred: {:?}", e), // Handle error case
/// }
/// ```
async fn delete(
    db_client: &db::DbClient,
    exam: &models::EditExam,
) -> Result<HttpResponse, AppError> {
    match fetch_ids_query(db_client, exam.exam_id.exam_id).await {
        Ok(result) => {
            let result_responses = generate_final_response(result);

            let deletion_data = models::DeletionData::new(
                exam.delete.section_ids.clone(),
                exam.delete.question_ids.clone(),
                exam.delete.option_ids.clone(),
                result_responses,
            );

            delete_query(db_client, deletion_data).await?; // Perform the deletion

            Ok(HttpResponse::Ok().finish()) // Return success response
        }
        Err(e) => Err(e), // Propagate error if any
    }
}

/// Updates sections in the database with new titles based on the provided input.
///
/// # Arguments
/// * `db_client` - A reference to the database client used to interact with the database.
/// * `input` - A vector of `Section` structs, each containing the section's ID and new title that needs to be updated.
///
/// # Returns
/// * `Result<(), AppError>` - A `Result` containing `Ok(())` on successful update, or an `AppError` if the operation fails.
///
/// # Example
/// ```rust
/// use crate::models::{Section};
/// use crate::db::DbClient;
/// use crate::errors::AppError;
///
/// let section_to_update = vec![
///     Section { id: 1, title: "New Title for Section 1".into() },
///     Section { id: 2, title: "Updated Section 2 Title".into() }
/// ];
///
/// let db_client = DbClient::new(); // Assuming you have a way to instantiate this
///
/// // Assuming async runtime context like actix-rt
/// let result = update_section(&db_client, &section_to_update).await;
///
/// match result {
///     Ok(_) => println!("Sections updated successfully"),
///     Err(AppError::DbErr(e)) => eprintln!("Database error: {}", e),
///     Err(_) => eprintln!("Unknown error"),
/// }
/// ```
async fn update_section(db_client: &db::DbClient, input: &Vec<Section>) -> Result<(), AppError> {
    let db_conn = &db_client.db;
    let mut update_builder = sections::Entity::update_many();

    for section_to_update in input {
        update_builder = update_builder
            .col_expr(
                sections::Column::Title,
                Expr::value(section_to_update.title.clone()),
            )
            .filter(Expr::col(sections::Column::Id).eq(section_to_update.id));
    }

    update_builder
        .exec(db_conn)
        .await
        .map_err(|e| AppError::DbErr(e))?;

    Ok(())
}

/// Updates multiple questions in the database based on the provided input vector.
///
/// Each question in the input vector is matched by its `id`, and the `text`, `description`,
/// and `marks` fields are updated accordingly.
///
/// # Arguments
/// * `db_client` - A reference to the database client (`DbClient`) that provides the database connection.
/// * `input` - A vector of `Question` structs, where each struct includes the ID of the question to update
///             and the new values for its fields.
///
/// # Returns
/// * `Ok(())` if all updates succeed.
/// * `Err(AppError)` if any database error occurs.
///
/// # Example
/// ```no_run
/// use crate::models::Question;
/// use crate::db::DbClient;
/// use crate::errors::AppError;
///
/// let questions_to_update = vec![
///     Question {
///         id: 1,
///         text: String::from("What is the capital of France?"),
///         description: Some(String::from("Geography-related question")),
///         marks: 5,
///     },
///     Question {
///         id: 2,
///         text: String::from("Solve 2 + 2"),
///         description: Some(String::from("Basic arithmetic question")),
///         marks: 2,
///     },
/// ];
///
/// let db_client = DbClient::new(); // assuming this returns a valid db client
///
/// actix_rt::spawn(async move {
///     match update_question(&db_client, &questions_to_update).await {
///         Ok(_) => println!("Questions updated successfully"),
///         Err(e) => eprintln!("Failed to update questions: {:?}", e),
///     }
/// });
/// ```
async fn update_question(db_client: &db::DbClient, input: &Vec<Question>) -> Result<(), AppError> {
    let db_conn = &db_client.db;
    let mut update_builder = questions::Entity::update_many();

    for question_to_update in input {
        update_builder = update_builder
            .col_expr(
                questions::Column::Text,
                Expr::value(question_to_update.text.clone()),
            )
            .col_expr(
                questions::Column::Description,
                Expr::value(question_to_update.description.clone()),
            )
            .col_expr(
                questions::Column::Marks,
                Expr::value(question_to_update.marks),
            )
            .filter(Expr::col(questions::Column::Id).eq(question_to_update.id));
    }

    update_builder
        .exec(db_conn)
        .await
        .map_err(|e| AppError::DbErr(e))?;

    Ok(())
}

/// Updates multiple options in the database based on the provided input vector.
///
/// Each `OptionModel` in the input vector is matched by its `id`, and the `text` and
/// `is_correct` fields are updated accordingly.
///
/// # Arguments
/// * `db_client` - A reference to the database client (`DbClient`) that holds the DB connection.
/// * `input` - A vector of `OptionModel` instances, each containing updated values for existing options.
///
/// # Returns
/// * `Ok(())` if all updates succeed.
/// * `Err(AppError)` if a database error occurs.
///
/// # Example
/// ```no_run
/// use crate::models::OptionModel;
/// use crate::db::DbClient;
/// use crate::errors::AppError;
///
/// let options_to_update = vec![
///     OptionModel {
///         id: 1,
///         text: String::from("Paris"),
///         is_correct: true,
///     },
///     OptionModel {
///         id: 2,
///         text: String::from("London"),
///         is_correct: false,
///     },
/// ];
///
/// let db_client = DbClient::new(); // Assume this returns a valid db client
///
/// actix_rt::spawn(async move {
///     match update_option(&db_client, &options_to_update).await {
///         Ok(_) => println!("Options updated successfully"),
///         Err(e) => eprintln!("Failed to update options: {:?}", e),
///     }
/// });
/// ```
pub async fn update_option(
    db_client: &db::DbClient,
    input: &Vec<OptionModel>,
) -> Result<(), AppError> {
    let db_conn = &db_client.db;
    let mut update_builder = options::Entity::update_many();

    for option_to_update in input {
        update_builder = update_builder
            .col_expr(
                options::Column::Text,
                Expr::value(option_to_update.text.clone()),
            )
            .col_expr(
                options::Column::IsCorrect,
                Expr::value(option_to_update.is_correct),
            )
            .filter(Expr::col(options::Column::Id).eq(option_to_update.id)); // Fixed: should reference options::Column::Id, not questions
    }

    update_builder
        .exec(db_conn)
        .await
        .map_err(AppError::DbErr)?;

    Ok(())
}

/// Handles the editing of an exam by processing updates and deletions for sections, questions, and options.
///
/// This endpoint accepts a JSON body containing the edits and deletions to apply.
/// It will only perform updates or deletions if the corresponding fields are non-empty.
///
/// # Arguments
/// * `app_state` - Shared application state, containing the database client.
/// * `req_body` - JSON payload with the `EditExam` structure.
///
/// # Returns
/// * `HttpResponse::Ok()` if the operation completes successfully.
/// * `AppError` if any step in the update or delete process fails.
///
/// # Behavior
/// - Updates sections, questions, and options if respective vectors are not empty.
/// - Performs deletions if the `delete` field contains non-empty ID lists.
///
/// # Example Request JSON:
/// ```json
/// {
///     "exam_id": { "exam_id": 1 },
///     "sections": [{ "id": 1, "title": "Updated Section" }],
///     "questions": [],
///     "options": [],
///     "delete": {
///         "section_ids": [],
///         "question_ids": [2],
///         "option_ids": []
///     }
/// }
/// ```
pub async fn edit_exam(
    app_state: web::Data<models::AppState>,
    req_body: web::Json<models::EditExam>,
) -> Result<HttpResponse, AppError> {
    // Update sections if provided
    if !req_body.sections.is_empty() {
        update_section(&app_state.db_client, &req_body.sections).await?;
    }

    // Update questions if provided
    if !req_body.questions.is_empty() {
        update_question(&app_state.db_client, &req_body.questions).await?;
    }

    // Update options if provided
    if !req_body.options.is_empty() {
        update_option(&app_state.db_client, &req_body.options).await?;
    }

    // Perform deletions if any ID list is non-empty
    if !req_body.delete.is_all_empty() {
        delete(&app_state.db_client, &req_body).await?;
    }

    Ok(HttpResponse::Ok().finish())
}
