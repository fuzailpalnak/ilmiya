use crate::{handlers::llm_response_handler::handle_llm_response, model::{self, request::PromptLanguage}};
use actix_web::{web, HttpResponse};
use anyhow::{Result};
use log::error;
use crate::utils;

fn build_mcq_prompt(question: &String, correct_answer: &String, language: PromptLanguage) -> String {

    match language {
        PromptLanguage::Arabic => utils::prompts::arabic_prompt_template_context_fill_in_the_blank(question, correct_answer),
        PromptLanguage::Urdu => utils::prompts::urdu_prompt_template_context_fill_in_the_blank(question, correct_answer),
    }

}

pub async fn generate_options_from_context(
    app_state: web::Data<model::state::AppState>,
    req_body: web::Json<model::request::SimilarFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let language = utils::parse::map_to_prompt_language(&req_body.language);
    let prompt = build_mcq_prompt(&req_body.question, &req_body.correct_answer, language);

    match handle_llm_response(&app_state.text_generation_api.get_url(), prompt, 1).await {
        Ok(raw_output) => match utils::parse::parse_similar_fill_in_the_blanks_options(&raw_output) {
            Ok(response) => Ok(HttpResponse::Ok().json(response)),
            Err(e) => {
                error!("Failed to parse MCQ options from the llm response: {:?}", e);
                Ok(HttpResponse::InternalServerError().body(format!("Parsing error: {}", e)))
            }
        },
        Err(e) => {
            error!("LLM API failure: {:?}", e);
            Ok(HttpResponse::InternalServerError().body(format!("LLM API error: {}", e)))
        }
    }
}
