
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub contents: Vec<Content>,
    pub generation_config: GenerationConfig,
}

#[derive(Serialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub candidate_count: u32,
    pub temperature: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub prompt_feedback: Option<PromptFeedback>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Option<ContentResponse>,
    pub finish_reason: Option<String>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

#[derive(Deserialize, Debug)]
pub struct ContentResponse {
    pub parts: Vec<PartResponse>,
}

#[derive(Deserialize, Debug)]
pub struct PartResponse {
    pub text: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PromptFeedback {
    pub block_reason: Option<String>,
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Serialize)]
pub struct Suggestion {
    pub suggestion: String,
}

#[derive(Serialize)]
pub struct NextWordResponse {
    pub suggestions: Vec<String>,
}

impl Content {
    pub fn new(text: String) -> Self {
        Self {
            parts: vec![Part { text }],
        }
    }
}

impl RequestBody {
    pub fn new(prompt: String, candidate_count: u32, temprature: f32) -> Self {
        Self {
            contents: vec![Content::new(prompt)],
            generation_config: GenerationConfig {
                candidate_count,
                temperature: Some(temprature),
            },
        }
    }
}