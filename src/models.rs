use crate::db;
use crate::entities::{details, options, questions, sections};
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub db_client: db::DbClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamDescription {
    pub title: String,
    pub description: String,
    pub duration: i32,
    pub passing_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub section_id: i32,
    pub text: String,
    pub description: String,
    pub marks: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Option {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamId {
    pub exam_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exam {
    pub exam_description: ExamDescription,
    pub sections: Vec<Section>,
    pub questions: Vec<Question>,
    pub options: Vec<Option>,
}

#[derive(Debug)]
pub struct QueryResponse {
    pub exam: details::Model,
    pub section: sections::Model,
    pub question: questions::Model,
    pub option: options::Model,
}

impl FromQueryResult for QueryResponse {
    fn from_query_result(res: &sea_orm::QueryResult, _pre: &str) -> Result<Self, sea_orm::DbErr> {
        let exam = parse_query_to_model::<details::Model, details::Entity>(res)?.into();
        let section = parse_query_to_model::<sections::Model, sections::Entity>(res)?;
        let question = parse_query_to_model::<questions::Model, questions::Entity>(res)?;
        let option = parse_query_to_model::<options::Model, options::Entity>(res)?;

        Ok(Self {
            exam,
            section,
            question,
            option,
        })
    }
}

pub fn parse_query_to_model<M: FromQueryResult, E: EntityTrait>(
    res: &sea_orm::QueryResult,
) -> Result<M, sea_orm::DbErr> {
    M::from_query_result(res, E::default().table_name())
}
