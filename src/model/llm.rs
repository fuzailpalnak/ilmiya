
use serde::{Serialize, Deserialize};

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
pub struct QuranicVerseFillInThBlankTextGenerationRequest {
    pub question: String,
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


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LLMResponse {
    pub candidates: Option<Vec<LLMOptionsResponse>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LLMOptionsResponse {
    pub content: Option<LLMContentResponse>,
}

#[derive(Deserialize, Debug)]
pub struct LLMContentResponse {
    pub parts: Vec<LLMPartResponse>,
}

#[derive(Deserialize, Debug)]
pub struct LLMPartResponse {
    pub text: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GuessFillInTheBlankResponse {
    pub responses: Vec<String>,
}
