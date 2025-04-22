use crate::model;
use anyhow::{Context, Result};

/// Function to delete an exam and automatically cascade delete related data.
pub async fn delete_exam(pool: &sqlx::PgPool, exam_id: i32) -> Result<()> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    sqlx::query!(
        r#"
        DELETE FROM exam
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

pub async fn delete_section_question_option_based_on_ids(
    pool: &sqlx::PgPool,
    deletion_data: &model::request::DeleteIdsRequest,
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
