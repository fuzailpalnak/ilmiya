use crate::model::response::ExamResponse;

use crate::database::schema;
use anyhow::{Context, Result};

use crate::utils::parse;

/// Fetches an exam by its ID from the `exam` table.
///
/// # Arguments
///
/// * `pool` - A reference to the SQLx connection pool.
/// * `exam_id` - The ID of the exam to retrieve.
///
/// # Example (non-runnable)
/// ```ignore
/// let exam = fetch_exam_id(&pool, 1).await?;
/// println!("Exam ID: {}", exam.id);
/// ```
async fn fetch_exam_id(pool: &sqlx::PgPool, exam_id: i32) -> Result<schema::ExamModel> {
    sqlx::query_as!(
        schema::ExamModel,
        r#"
        SELECT id
        FROM exam
        WHERE id = $1
        "#,
        exam_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to fetch exam id")
}

/// Fetches the description of an exam from the `details` table.
///
/// # Arguments
///
/// * `pool` - A reference to the SQLx connection pool.
/// * `exam_id` - The ID of the exam to get details for.
///
/// # Example (non-runnable)
/// ```ignore
/// let description = fetch_exam_description(&pool, 1).await?;
/// println!("Title: {}", description.title);
/// ```
async fn fetch_exam_description(
    pool: &sqlx::PgPool,
    exam_id: i32,
) -> Result<schema::ExamDescriptionModel> {
    sqlx::query_as!(
        schema::ExamDescriptionModel,
        r#"
        SELECT
            id,
            exam_id,
            title,
            description,
            duration,
            passing_score
        FROM details
        WHERE exam_id = $1
        "#,
        exam_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to fetch exam description")
}

/// Fetches all sections, questions, and options related to an exam ID.
/// This joins the `exam`, `details`, `sections`, `questions`, and `options` tables.
///
/// # Arguments
///
/// * `pool` - A reference to the SQLx connection pool.
/// * `exam_id` - The ID of the exam to retrieve content for.
///
/// # Example (non-runnable)
/// ```ignore
/// let sections = fetch_sections_and_questions(&pool, 1).await?;
/// for row in &sections {
///     println!("Section: {}", row.section_title);
/// }
/// ```
async fn fetch_sections_and_questions(
    pool: &sqlx::PgPool,
    exam_id: i32,
) -> Result<Vec<schema::SectionRow>> {
    sqlx::query_as!(
        schema::SectionRow,
        r#"
        SELECT
            s.id AS section_id,
            s.title AS section_title,
            s.details_id AS section_details_id,
            q.id AS question_id,
            q.text AS question_text,
            q.description AS question_description,
            q.marks AS question_marks,
            o.id AS option_id,
            o.text AS option_text,
            o.is_correct AS option_is_correct
        FROM exam e
        JOIN details d ON e.id = d.exam_id
        JOIN sections s ON d.id = s.details_id
        LEFT JOIN questions q ON s.id = q.section_id
        LEFT JOIN options o ON q.id = o.question_id
        WHERE e.id = $1
        "#,
        exam_id
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch sections and questions")
}

/// Reads the full data for a given exam, including basic info, description,
/// and all related sections with questions and options.
///
/// This aggregates all parts of the exam into a single `ExamResponse`
/// object, fetching from multiple tables and parsing the results.
///
/// # Arguments
///
/// * `pool` - The SQLx database connection pool.
/// * `exam_id` - The ID of the exam to retrieve.
///
/// # Returns
///
/// An `ExamResponse` with the full hierarchical exam data.
///
/// # Example (non-runnable)
/// ```ignore
/// let response = read_exam_data(&pool, 1).await?;
/// println!("Exam ID: {:?}", response.exam_id);
/// for section in response.sections {
///     println!("Section: {}", section.base.title);
///     for question in section.questions {
///         println!("Question: {}", question.base.text);
///     }
/// }
/// ```
pub async fn read_exam_data(pool: &sqlx::PgPool, exam_id: i32) -> Result<ExamResponse> {
    let exam_model = fetch_exam_id(pool, exam_id).await?;
    let exam_description = fetch_exam_description(pool, exam_id).await?;
    let sections = fetch_sections_and_questions(pool, exam_id).await?;
    let sections_map = parse::map_to_section_response(sections)?;
    let sections = sections_map.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    Ok(ExamResponse {
        exam_id: exam_model.into(),
        exam_description: exam_description.into(),
        sections,
    })
}
