use serde::Deserialize;
use std::fs;
use once_cell::sync::Lazy;
use std::env;

#[derive(Deserialize, Debug)]
pub struct PromptTemplates {
    pub prompt_context_urdu: String,
    pub prompt_quranic_verse: String,
}

pub static PROMPT_TEMPLATES: Lazy<PromptTemplates> = Lazy::new(|| {
    dotenv::dotenv().ok();
    let path = env::var("PROMPT_TEMPLATE_PATH")
        .expect("PROMPT_TEMPLATE_PATH not set in .env file");

    let file_content = fs::read_to_string(path).expect("Failed to read prompt template file");
    serde_json::from_str(&file_content).expect("Failed to parse prompt template JSON")
    
});


pub fn arabic_prompt_template_quranic_verse_fill_in_the_blank(question: &String,correct_answer: &String) -> String {
    PROMPT_TEMPLATES
        .prompt_quranic_verse
        .replace("{question}", question.trim())
        .replace("{correct_answer}", correct_answer.trim())
}

// pub fn arabic_prompt_template_context_fill_in_the_blank(question: &String, correct_answer: &String) -> String {
//     PROMPT_TEMPLATES
//         .arabic_context_fill
//         .replace("{question}", question.trim())
//         .replace("{correct_answer}", correct_answer.trim())
// }

pub fn urdu_prompt_template_context_fill_in_the_blank(question: &String, correct_answer: &String) -> String {
    PROMPT_TEMPLATES
        .prompt_context_urdu
        .replace("{question}", question.trim())
        .replace("{correct_answer}", correct_answer.trim())
}
