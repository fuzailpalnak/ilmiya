use crate::model;
use anyhow::{Context, Result};

/// Deletes an exam and all related data (via cascading or manual deletion depending on schema).
///
/// This function starts a database transaction and deletes the exam record
/// from the `exam` table. If foreign key constraints are set up with `ON DELETE CASCADE`,
/// related rows in `details`, `sections`, `questions`, and `options` will be deleted automatically.
///
/// # Arguments
///
/// * `pool` - A reference to the SQLx PostgreSQL connection pool.
/// * `exam_id` - The ID of the exam to delete.
///
/// # Errors
///
/// Returns an error if the transaction fails to begin, if the deletion fails,
/// or if the transaction fails to commit.
///
/// # Example (non-runnable)
/// ```ignore
/// delete_exam(&pool, 1).await?;
/// println!("Exam deleted successfully.");
/// ```
pub async fn delete_exam(pool: &sqlx::PgPool, exam_id: i32) -> Result<()> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    sqlx::query!(
        r#"
        DELETE FROM exams
        WHERE id = $1
        "#,
        exam_id
    )
    .execute(&mut *tx)
    .await
    .context("Failed to delete exam")?;

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}

/// Deletes specific sections, questions, and options from the database.
///
/// This function allows you to manually delete related entities from their respective
/// tables (`sections`, `questions`, and `options`) in a single transaction.
/// The deletions are performed in the order: `sections`, `questions`, then `options`.
///
/// **Note:** Ensure the provided IDs are valid and correspond to existing entities.
///
/// # Arguments
///
/// * `pool` - A reference to the SQLx PostgreSQL connection pool.
/// * `deletion_data` - A `DeleteIdsRequest` containing vectors of IDs for
///   sections, questions, and options to delete.
///
/// # Errors
///
/// Returns an error if the transaction fails to begin, if any deletion query fails,
/// or if the transaction fails to commit.
///
/// # Example (non-runnable)
/// ```ignore
/// let deletion_data = DeleteIdsRequest {
///     section_ids: vec![1, 2],
///     question_ids: vec![10, 11],
///     option_ids: vec![100, 101],
/// };
/// delete_related_entities(&pool, &deletion_data).await?;
/// println!("Related entities deleted successfully.");
/// ```
pub async fn delete_related_entities(
    pool: &sqlx::PgPool,
    deletion_data: &model::delete::DeleteIdsRequest,
) -> Result<()> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    let section_ids: Vec<i32> = deletion_data.section_ids.iter().copied().collect();
    sqlx::query!(
        r#"
        DELETE FROM sections
        WHERE id = ANY($1);
        "#,
        &section_ids
    )
    .execute(&mut *tx)
    .await
    .context("Failed to delete sections")?;

    let question_ids: Vec<i32> = deletion_data.question_ids.iter().copied().collect();

    sqlx::query!(
        r#"
        DELETE FROM questions
        WHERE id = ANY($1);
        "#,
        &question_ids
    )
    .execute(&mut *tx)
    .await
    .context("Failed to delete questions")?;

    let option_ids: Vec<i32> = deletion_data.option_ids.iter().copied().collect();

    sqlx::query!(
        r#"
            DELETE FROM options
            WHERE id = ANY($1);
            "#,
        &option_ids
    )
    .execute(&mut *tx)
    .await
    .context("Failed to delete options")?;

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}
