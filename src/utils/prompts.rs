use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;
use std::fs;

use crate::model::llm::DistractorType;

#[derive(Deserialize, Debug)]
pub struct PromptTemplates {
    pub prompt_context_urdu: String,
    pub prompt_quranic_verse: String,
    pub prompt_quranic_verse_distractor_collection: String,
    pub prompt_quranic_verse_diacritic_distractor: String,
    pub prompt_quranic_verse_phonetic_distractor: String,
    pub prompt_quranic_verse_morfological_distractor: String,
    pub prompt_quranic_verse_grammatical_distractor: String,
    pub prompt_quranic_verse_alternate_verse_distractor: String,
    pub prompt_quranic_verse_thematic_distractor: String,
    pub prompt_quranic_verse_collocational_distractor: String,
}

pub static PROMPT_TEMPLATES: Lazy<PromptTemplates> = Lazy::new(|| {
    dotenv::dotenv().ok();
    let path = env::var("PROMPT_TEMPLATE_PATH").expect("PROMPT_TEMPLATE_PATH not set in .env file");

    let file_content = fs::read_to_string(path).expect("Failed to read prompt template file");
    serde_json::from_str(&file_content).expect("Failed to parse prompt template JSON")
});

pub fn arabic_prompt_template_quranic_verse_distractor_mcq(
    question: &String,
    correct_answer: &String,
    distractor_type: DistractorType,
) -> String {
    let template = match distractor_type {
        DistractorType::Collection => &PROMPT_TEMPLATES.prompt_quranic_verse_distractor_collection,
        DistractorType::Diacritic => &PROMPT_TEMPLATES.prompt_quranic_verse_diacritic_distractor,
        DistractorType::Phonetic => &PROMPT_TEMPLATES.prompt_quranic_verse_phonetic_distractor,
        DistractorType::Morphological => {
            &PROMPT_TEMPLATES.prompt_quranic_verse_morfological_distractor
        }
        DistractorType::Grammatical => {
            &PROMPT_TEMPLATES.prompt_quranic_verse_grammatical_distractor
        }
        DistractorType::AlternateVerse => {
            &PROMPT_TEMPLATES.prompt_quranic_verse_alternate_verse_distractor
        }
        DistractorType::Thematic => &PROMPT_TEMPLATES.prompt_quranic_verse_thematic_distractor,
        DistractorType::Collocational => {
            &PROMPT_TEMPLATES.prompt_quranic_verse_collocational_distractor
        }
    };

    template
        .replace("{question}", question.trim())
        .replace("{correct_answer}", correct_answer.trim())
}

// pub fn arabic_prompt_template_context_fill_in_the_blank(question: &String, correct_answer: &String) -> String {
//     PROMPT_TEMPLATES
//         .arabic_context_fill
//         .replace("{question}", question.trim())
//         .replace("{correct_answer}", correct_answer.trim())
// }

pub fn urdu_prompt_template_context_mcq(question: &String, correct_answer: &String) -> String {
    PROMPT_TEMPLATES
        .prompt_context_urdu
        .replace("{question}", question.trim())
        .replace("{correct_answer}", correct_answer.trim())
}
