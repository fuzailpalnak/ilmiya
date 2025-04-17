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

fn generate_final_response(
    input: Vec<(Option<i32>, Option<i32>, Option<i32>)>,
) -> Vec<(i32, i32, i32)> {
    input
        .into_iter()
        .map(|(a, b, c)| (a.unwrap_or(0), b.unwrap_or(0), c.unwrap_or(0)))
        .collect()
}

async fn delete_query(
    db_client: &db::DbClient,
    mut deletion_data: models::DeletionData,
) -> Result<(), AppError> {
    let db_conn = &db_client.db;

    deletion_data.filter_deletions();

    // If section_ids are provided, let cascade handle deletion of related questions/options
    if !deletion_data.section_id_to_delete.is_empty() {
        sections::Entity::delete_many()
            .filter(
                Expr::col(sections::Column::Id).is_in(deletion_data.section_id_to_delete.clone()),
            )
            .exec(db_conn)
            .await
            .map_err(|e| AppError::DbErr(e))?;
    }

    if !deletion_data.question_id_to_delete.is_empty() {
        questions::Entity::delete_many()
            .filter(
                Expr::col(questions::Column::Id).is_in(deletion_data.question_id_to_delete.clone()),
            )
            .exec(db_conn)
            .await
            .map_err(|e| AppError::DbErr(e))?;
    }

    if !deletion_data.option_id_to_delete.is_empty() {
        options::Entity::delete_many()
            .filter(Expr::col(options::Column::Id).is_in(deletion_data.option_id_to_delete.clone()))
            .exec(db_conn)
            .await
            .map_err(|e| AppError::DbErr(e))?;
    }

    Ok(())
}

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

            delete_query(db_client, deletion_data).await?;

            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => Err(e),
    }
}

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
            ) // Using col_expr
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
            .filter(Expr::col(questions::Column::Id).eq(option_to_update.id));
    }

    update_builder
        .exec(db_conn)
        .await
        .map_err(|e| AppError::DbErr(e))?;

    Ok(())
}

/// Handles the HTTP request to edit an exam.
pub async fn edit_exam(
    app_state: web::Data<models::AppState>,
    req_body: web::Json<models::EditExam>,
) -> Result<HttpResponse, AppError> {
    // Only update section if it's not empty
    if !req_body.sections.is_empty() {
        update_section(&app_state.db_client, &req_body.sections).await?;
    }

    // Only update question if it's not empty
    if !req_body.questions.is_empty() {
        update_question(&app_state.db_client, &req_body.questions).await?;
    }

    // Only update options if it's not empty
    if !req_body.options.is_empty() {
        update_option(&app_state.db_client, &req_body.options).await?;
    }

    // Only delete if delete IDs are not empty or valid
    if !req_body.delete.is_all_empty() {
        delete(&app_state.db_client, &req_body).await?;
    }

    // Return Ok if everything is processed
    Ok(HttpResponse::Ok().finish())
}
