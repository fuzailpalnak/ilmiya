use crate::{services::llm::send_prompt_to_llm, model::{self, request::PromptLanguage}};
use actix_web::{web, HttpResponse};
use log::error;
use crate::utils;
use anyhow::{Result};

pub fn build_contextual_mcq_prompt(question: &String, correct_answer: &String, language: PromptLanguage) -> String {
    match language {
        PromptLanguage::Arabic => utils::prompts::arabic_prompt_template_context_fill_in_the_blank(question, correct_answer),
        PromptLanguage::Urdu => utils::prompts::urdu_prompt_template_context_fill_in_the_blank(question, correct_answer),
    }
}

pub fn build_similarity_mcq_prompt(correct_answer: &String, language: PromptLanguage) -> String {
    match language {
        PromptLanguage::Arabic => utils::prompts::arabic_prompt_template_similar_fill_in_the_blank(correct_answer),
        PromptLanguage::Urdu => String::from("Urdu prompt not yet supported."),
    }
}

pub async fn generate_mcq_options_from_context(
    app_state: web::Data<model::state::AppState>,
    req_body: web::Json<model::request::ContextFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let language = utils::parse::map_to_prompt_language(&req_body.language);
    let prompt = build_contextual_mcq_prompt(&req_body.question, &req_body.correct_answer, language);

    let raw_output = send_prompt_to_llm(&app_state.text_generation_api.get_url(), prompt, 1)
        .await
        .map_err(|e| {
            error!("LLM API failure: {:?}", e);
            actix_web::error::ErrorInternalServerError(format!("LLM API Error: {}", e))

        })?;

    let response = utils::parse::parse_similar_fill_in_the_blanks_options(&raw_output)
        .map_err(|e| {
            error!("Failed to parse MCQ options from the LLM response: {:?}", e);
            actix_web::error::ErrorInternalServerError(format!("Parsing error: {}", e))

        })?;

    Ok(HttpResponse::Ok().json(response))
}

pub async fn generate_mcq_options_for_similarity_quranic_verses(
    app_state: web::Data<model::state::AppState>,
    req_body: web::Json<model::request::SimilarFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let language = PromptLanguage::Arabic;
    let prompt = build_similarity_mcq_prompt(&req_body.correct_answer, language);

    let raw_output = send_prompt_to_llm(&app_state.text_generation_api.get_url(), prompt, 1)
        .await
        .map_err(|e| {
            error!("LLM API failure: {:?}", e);
            actix_web::error::ErrorInternalServerError(format!("LLM API error: {}", e))
        })?;

    let response = utils::parse::parse_similar_fill_in_the_blanks_options(&raw_output)
        .map_err(|e| {
            error!("Failed to parse MCQ options from the LLM response: {:?}", e);
            actix_web::error::ErrorInternalServerError(format!("Parsing error: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(response))
}
