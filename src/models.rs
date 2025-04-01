use crate::db;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamMetadata {
    pub exam_name: String,
    pub details: String,
    pub duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionedQuestions {
    pub sections: HashMap<String, Vec<QuestionResponse>>,
}

pub struct AppState {
    pub redis_client: db::RedisClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exam {
    pub exam_name: String,
    pub details: String,
    pub duration: u32,
    pub questions: HashMap<String, Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamResponse {
    pub exam_name: String,
    pub details: String,
    pub duration: u32,
    pub sections: SectionedQuestions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionResponse {
    pub question: String,
    pub options: Vec<String>,
    pub correct_option: u32,
    pub marks: u32,
    pub section: String,
    pub question_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
    pub correct_option: u32,
    pub marks: u32,
    pub section: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamId {
    pub exam_id: String,
}
