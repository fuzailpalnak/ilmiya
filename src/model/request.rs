use std::collections::HashSet;

use crate::database::schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamIdRequestModel {
    #[serde(flatten)]
    pub base: schema::ExamModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamDescriptionRequest {
    #[serde(flatten)]
    pub base: schema::ExamDescriptionModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionRequest {
    #[serde(flatten)]
    pub base: schema::SectionsModel,
    pub questions: Vec<QuestionRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionRequest {
    #[serde(flatten)]
    pub base: schema::QuestionsModel,
    pub options: Vec<OptionRequestModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionRequestModel {
    #[serde(flatten)]
    pub base: schema::OptionsModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamRequest {
    pub exam_id: ExamIdRequestModel,
    pub description: ExamDescriptionRequest,
    pub sections: Vec<SectionRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditExamRequest {
    pub exam_id: ExamIdRequestModel,
    pub description: ExamDescriptionRequest,
    pub sections: Vec<SectionRequest>,
    pub delete: DeleteIdsRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteIdsRequest {
    pub section_ids: Vec<i32>,
    pub question_ids: Vec<i32>,
    pub option_ids: Vec<i32>,
}

impl DeleteIdsRequest {
    pub fn is_all_empty(&self) -> bool {
        self.section_ids.is_empty() && self.question_ids.is_empty() && self.option_ids.is_empty()
    }
}
