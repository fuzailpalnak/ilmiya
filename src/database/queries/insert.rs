use crate::model::request;
use anyhow::{Context, Result};
use sqlx::PgConnection;

async fn insert_exam_id(tx: &mut PgConnection, req: &request::ExamRequest) -> Result<i32> {
    let result = sqlx::query!(
        r#"INSERT INTO exam (id, created_at, updated_at) VALUES ($1, DEFAULT, DEFAULT) RETURNING id"#,
        req.exam_id.base.id,
    )
    .fetch_one(&mut *tx)
    .await
    .context("Failed to insert into exam")?;

    Ok(result.id)
}

async fn insert_details(tx: &mut PgConnection, req: &request::ExamRequest) -> Result<i32> {
    let result = sqlx::query!(
        r#"
        INSERT INTO details (id, exam_id, title, description, duration, passing_score, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, DEFAULT, DEFAULT)
        RETURNING id
        "#,
        req.description.base.id,
        req.exam_id.base.id,
        req.description.base.title,
        req.description.base.description,
        req.description.base.duration,
        req.description.base.passing_score
    )
    .fetch_one(&mut *tx)
    .await
    .context("Failed to insert into details")?;

    Ok(result.id)
}

async fn insert_sections(
    tx: &mut PgConnection,
    section_ids: &[i32],
    detail_ids: &[i32],
    section_titles: &[String],
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO sections (id, details_id, title)
        SELECT * FROM UNNEST($1::int[], $2::int[], $3::text[])
        ON CONFLICT (id) DO NOTHING
        "#,
        section_ids,
        detail_ids,
        section_titles
    )
    .execute(&mut *tx)
    .await
    .context("Failed to insert sections")?;

    Ok(())
}

async fn insert_questions(
    tx: &mut PgConnection,
    question_ids: &[i32],
    section_ids: &[i32],
    texts: &[String],
    descs: &[String],
    marks: &[i32],
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO questions (id, section_id, text, description, marks)
        SELECT * FROM UNNEST($1::int[], $2::int[], $3::text[], $4::text[], $5::int[])
        ON CONFLICT (id) DO NOTHING
        "#,
        question_ids,
        section_ids,
        texts,
        descs,
        marks
    )
    .execute(&mut *tx)
    .await
    .context("Failed to insert questions")?;

    Ok(())
}

async fn insert_options(
    tx: &mut PgConnection,
    option_ids: &[i32],
    question_ids: &[i32],
    texts: &[String],
    correct_flags: &[bool],
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO options (id, question_id, text, is_correct)
        SELECT * FROM UNNEST($1::int[], $2::int[], $3::text[], $4::bool[])
        ON CONFLICT (id) DO NOTHING
        "#,
        option_ids,
        question_ids,
        texts,
        correct_flags
    )
    .execute(&mut *tx)
    .await
    .context("Failed to insert options")?;

    Ok(())
}

pub async fn insert_exam(pool: &sqlx::PgPool, exam: &request::ExamRequest) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .context("Failed to start DB transaction")?;

    let _exam_id = insert_exam_id(&mut tx, exam)
        .await
        .context("Failed to insert exam")?;

    let _detail_id = insert_details(&mut tx, exam)
        .await
        .context("Failed to insert exam details")?;

    let section_ids: Vec<i32> = exam.sections.iter().map(|s| s.base.id).collect();
    let section_titles: Vec<String> = exam.sections.iter().map(|s| s.base.title.clone()).collect();
    let detail_ids: Vec<i32> = vec![exam.description.base.id; section_ids.len()];

    insert_sections(&mut tx, &section_ids, &detail_ids, &section_titles)
        .await
        .context("Failed to insert sections")?;

    let mut question_ids = Vec::new();
    let mut question_section_ids = Vec::new();
    let mut question_texts = Vec::new();
    let mut question_descs = Vec::new();
    let mut question_marks = Vec::new();

    let mut option_ids = Vec::new();
    let mut option_question_ids = Vec::new();
    let mut option_texts = Vec::new();
    let mut option_correct_flags = Vec::new();

    for section in &exam.sections {
        for q in &section.questions {
            question_ids.push(q.base.id);
            question_section_ids.push(q.base.section_id);
            question_texts.push(q.base.text.clone());
            question_descs.push(q.base.description.clone().unwrap_or_default());
            question_marks.push(q.base.marks);

            for opt in &q.options {
                option_ids.push(opt.base.id);
                option_question_ids.push(opt.base.question_id);
                option_texts.push(opt.base.text.clone());
                option_correct_flags.push(opt.base.is_correct.unwrap_or_default());
            }
        }
    }

    insert_questions(
        &mut tx,
        &question_ids,
        &question_section_ids,
        &question_texts,
        &question_descs,
        &question_marks,
    )
    .await
    .context("Failed to insert questions")?;

    insert_options(
        &mut tx,
        &option_ids,
        &option_question_ids,
        &option_texts,
        &option_correct_flags,
    )
    .await
    .context("Failed to insert options")?;

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}
