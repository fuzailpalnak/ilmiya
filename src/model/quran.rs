use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuranApiRequest {
    pub surah: u32,
    pub verse: u32,
}

#[derive(serde::Serialize)]
pub struct QuranApiRedisResponse {
    pub text: Vec<String>,
    pub mode: String,
}
