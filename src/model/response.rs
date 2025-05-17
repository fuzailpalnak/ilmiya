use crate::database::schema;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

impl From<schema::ExamDescriptionModel> for ExamDescription {
    fn from(model: schema::ExamDescriptionModel) -> Self {
        ExamDescription {
            id: model.id,
            exam_id: model.exam_id,
            title: model.title,
            description: model.description.unwrap_or_default(),
            duration: model.duration,
            passing_score: model.passing_score,
        }
    }
}

impl From<schema::ExamModel> for ExamIdResponse {
    fn from(model: schema::ExamModel) -> Self {
        ExamIdResponse { id: model.id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamDescription {
    pub id: i32,
    pub exam_id: i32,
    pub title: String,
    pub description: String,
    pub duration: i32,
    pub passing_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamIdResponse {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct SectionResponse {
    #[serde(flatten)]
    pub base: schema::SectionsModel,
    pub questions: Vec<QuestionResponse>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct QuestionResponse {
    #[serde(flatten)]
    pub base: schema::QuestionsModel,
    pub options: Vec<OptionResponseModel>,
}

#[derive(Debug, Serialize)]
pub struct OptionResponseModel {
    #[serde(flatten)]
    pub base: schema::OptionsModel,
}

#[derive(Debug, Serialize)]
pub struct ExamIdResponseModel {
    #[serde(flatten)]
    pub base: schema::ExamModel,
}

#[derive(Serialize, Debug)]
pub struct ExamResponse {
    pub exam_id: ExamIdResponse,
    pub description: ExamDescription,
    pub sections: Vec<SectionResponse>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GuessFillInTheBlankResponse {
    pub responses: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ResponseWrapper {
    responses: Vec<String>,
}
