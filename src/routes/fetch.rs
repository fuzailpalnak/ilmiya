use crate::db;

use crate::entities::{details, exam, options, questions, sections};
use crate::errors::AppError;
use crate::models;
use actix_web::{web, HttpResponse};

use sea_orm::sea_query::JoinType;
use sea_orm::QuerySelect;
use sea_orm::RelationTrait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use std::collections::HashMap;

/// Runs the query to retrieve the exam details along with associated sections, questions, options, and correct options.
/// This function interacts with the database and returns the raw data.
///
/// # Arguments
/// * `db_client` - A reference to the database client to interact with the database.
/// * `exam_id` - The ID of the exam to be fetched.
///
/// # Returns
/// * `Result<Vec<models::QueryResponse>, AppError>` - A result containing the query response data or an error if the operation fails.
///
/// # Example
/// ```rust
/// let result = fetch_exam_details(&db_client, 1).await;
/// match result {
///     Ok(response) => println!("Fetched exam details: {:?}", response),
///     Err(e) => println!("Error fetching exam details: {}", e),
/// }
/// ```
async fn fetch_exam_query(
    db_client: &db::DbClient,
    exam_id: i32,
) -> Result<Vec<models::QueryResponse>, AppError> {
    let db_conn = &db_client.db;

    let select = exam::Entity::find()
        .filter(exam::Column::Id.eq(exam_id))
        .select_only(); // Select only the specified columns

    let result = db::Prefixer::new(select)
        .add_columns(exam::Entity)
        .add_columns(details::Entity)
        .add_columns(sections::Entity)
        .add_columns(questions::Entity)
        .add_columns(options::Entity)
        .selector
        .join(JoinType::Join, exam::Relation::Details.def())
        .join(JoinType::Join, details::Relation::Sections.def())
        .join(JoinType::Join, sections::Relation::Questions.def())
        .join(JoinType::Join, questions::Relation::Options.def())
        .into_model::<models::QueryResponse>()
        .all(db_conn)
        .await?;

    Ok(result)
}

/// Retrieves the details of a specific exam by calling the query function and returning the data in a JSON response.
/// It processes the query result and returns it as an HTTP response.
///
/// # Arguments
/// * `db_client` - A reference to the database client to interact with the database.
/// * `exam_id` - The ID of the exam to be fetched.
///
/// # Returns
/// * `HttpResponse` - An HTTP response containing the exam data in JSON format.
///
/// # Example
/// ```rust
/// let result = retrive(&db_client, 1).await;
/// match result {
///     Ok(response) => println!("Exam details: {:?}", response),
///     Err(e) => println!("Error fetching exam: {}", e),
/// }
/// ```
async fn retrive(db_client: &db::DbClient, exam_id: i32) -> Result<HttpResponse, AppError> {
    match fetch_exam_query(db_client, exam_id).await {
        Ok(result) => Ok(HttpResponse::Ok().json(generate_final_response(result))),
        Err(e) => Err(e),
    }
}

/// Transforms a vector of `Response` objects into a structured `Exam1` format.
///
/// This function processes a list of `Response` instances—each representing part of a full exam—and
/// organizes them into a complete `Exam1` model with an exam description, sections, questions, and options.
///
/// # Arguments
///
/// * `responses` - A vector of `Response` instances where each entry includes an exam, section, question, and option.
///
/// # Returns
///
/// A `models::Exam1` struct composed of:
/// - `exam_description`: General metadata about the exam (title, duration, passing score, etc.).
/// - `sections`: Unique sections referenced in the exam.
/// - `questions`: Unique questions belonging to those sections.
/// - `options`: All options for all questions, preserving correctness information.
///
/// # Panics
///
/// This function will panic if no `Response` contains an exam description (i.e., if `responses` is empty).
///
/// # Example
///
/// ```
/// let responses = vec![/* fill with mock Response data */];
/// let exam = generate_final_response(responses);
/// assert_eq!(exam.sections.len(), 2);
fn generate_final_response(responses: Vec<models::QueryResponse>) -> models::Exam {
    let mut exam_description = None;
    let mut sections_map = HashMap::new();
    let mut questions_map = HashMap::new();
    let mut options = Vec::new();

    // Iterate over the responses to extract the necessary data
    for response in responses {
        // Extract exam details
        if exam_description.is_none() {
            exam_description = Some(models::ExamDescription {
                title: response.exam.title,
                description: response.exam.description.unwrap_or_default(),
                duration: response.exam.duration,
                passing_score: response.exam.passing_score,
            });
        }

        // Collect sections
        let section = response.section;
        sections_map.insert(
            section.id,
            models::Section {
                id: section.id,
                title: section.title.clone(),
            },
        );

        // Collect questions
        let question = response.question;
        questions_map.insert(
            question.id,
            models::Question {
                id: question.id,
                section_id: question.section_id,
                text: question.text.clone(),
                description: question.description.unwrap_or_default().clone(),
                marks: question.marks,
            },
        );

        // Collect options
        let option = response.option;
        options.push(models::OptionModel {
            id: option.id,
            question_id: option.question_id,
            text: option.text.clone(),
            is_correct: option.is_correct.unwrap_or(false),
        });
    }

    // Finalize the sections and questions into Vecs
    let sections: Vec<models::Section> = sections_map.into_iter().map(|(_, v)| v).collect();
    let questions: Vec<models::Question> = questions_map.into_iter().map(|(_, v)| v).collect();

    // Generate the final response
    models::Exam {
        exam_description: exam_description.unwrap(), // We assume there is at least one exam
        sections,
        questions,
        options,
    }
}

/// Actix-web handler to fetch the exam data based on the provided `exam_id`.
///
/// This function receives the exam ID as a path parameter, parses it, and calls the `retrive` function
/// to fetch the exam details. If the ID is valid, it will return the exam details as a JSON response.
/// Otherwise, it returns an error response.
///
/// # Arguments
///
/// * `app_state` - An Actix-web `Data` containing the app's state, which includes the database client.
/// * `exam_id` - The ID of the exam to be fetched, passed as a path parameter.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the exam data if successful. If the exam ID is invalid,
/// an error response is returned.
///
/// # Example
///
/// ```rust
/// let response = fetch_exam(app_state, "1".into()).await;
/// match response {
///     Ok(resp) => println!("Exam data: {:?}", resp),
///     Err(e) => println!("Error fetching exam: {}", e),
/// }
/// ```
pub async fn fetch_exam(
    app_state: web::Data<models::AppState>,
    exam_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let exam_id_int: i32 = exam_id
        .into_inner()
        .parse()
        .map_err(|_| AppError::NotFound("Invalid exam ID".into()))?;

    // Call the retrive function to get exam data
    retrive(&app_state.db_client, exam_id_int).await
}
