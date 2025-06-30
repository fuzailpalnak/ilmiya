use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ExamDescriptionModel {
    pub id: i32,
    pub exam_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub duration: i32,
    pub passing_score: i32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct CorrectOptionModel {
    pub option_id: i32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ExamModel {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct OptionsModel {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub is_correct: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct QuestionsModel {
    pub id: i32,
    pub section_id: i32,
    pub text: String,
    pub description: Option<String>,
    pub marks: i32,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct SectionsModel {
    pub id: i32,
    pub details_id: i32,
    pub title: String,
}

#[derive(sqlx::FromRow)]
pub struct SectionRow {
    pub section_id: i32,
    pub section_title: String,
    pub section_details_id: i32,
    pub question_id: i32,
    pub question_text: String,
    pub question_description: Option<String>,
    pub question_marks: i32,
    pub option_id: i32,
    pub option_text: String,
    pub option_is_correct: Option<bool>,
}
