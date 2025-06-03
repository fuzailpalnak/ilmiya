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


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Arabic,
    Urdu,
}

pub enum PromptLanguage {
    Arabic,
    Urdu,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextFillInThBlankTextGenerationRequest {
    pub question: String,
    pub correct_answer: String,
    pub language: Language, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarFillInThBlankTextGenerationRequest {
    pub correct_answer: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LLMRequest {
    pub contents: Vec<LLMContent>,
    pub generation_config: LLMGenerationConfig,
}

#[derive(Serialize, Debug)]
pub struct LLMContent {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LLMGenerationConfig {
    pub candidate_count: u32,
    pub temperature: Option<f32>,
}

impl LLMContent {
    pub fn new(text: String) -> Self {
        Self {
            parts: vec![Part { text }],
        }
    }
}

impl LLMRequest {
    pub fn new(prompt: String, candidate_count: u32, temprature: f32) -> Self {
        Self {
            contents: vec![LLMContent::new(prompt)],
            generation_config: LLMGenerationConfig {
                candidate_count,
                temperature: Some(temprature),
            },
        }
    }
}