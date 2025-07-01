use crate::database::schema;
use crate::model::question::{QuestionRequest, QuestionResponse};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SectionRequest {
    #[serde(flatten)]
    pub base: schema::SectionsModel,
    pub questions: Vec<QuestionRequest>,
}

#[derive(Debug, Serialize)]
pub struct SectionResponse {
    #[serde(flatten)]
    pub base: schema::SectionsModel,
    pub questions: Vec<QuestionResponse>,
}
