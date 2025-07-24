use crate::model::llm::{
    AlternateVerseDistractorResponse, CollocationalDistractorResponse, DiacriticDistractorResponse,
    DistractorType, GrammaticalDistractorResponse,
    GuessFillInTheBlankQuranDistractorCollectionResponse, GuessFillInTheBlankResponse,
    MorphologicalDistractorResponse, PhoneticOrthographicDistractorResponse,
    ThematicDistractorResponse,
};
use crate::utils;
use crate::{
    model::{self, llm::PromptLanguage},
    services::llm::send_prompt_to_llm,
};
use actix_web::{web, HttpResponse};
use anyhow::Result;
use log::error;
use serde::Serialize;

use serde::de::DeserializeOwned;

pub trait QuranDistractorResponse: DeserializeOwned + Send + 'static {}
impl<T: DeserializeOwned + Send + 'static> QuranDistractorResponse for T {}

pub fn build_contextual_mcq_prompt(
    question: &String,
    correct_answer: &String,
    language: PromptLanguage,
) -> Result<String, actix_web::Error> {
    match language {
        PromptLanguage::Arabic => Err(actix_web::error::ErrorBadRequest(
            "Arabic language is not supported for this endpoint",
        )),
        PromptLanguage::Urdu => Ok(utils::prompts::urdu_prompt_template_context_mcq(
            question,
            correct_answer,
        )),
    }
}

pub fn get_quranic_verse_distractor_prompt(
    question: &String,
    correct_answer: &String,
    language: PromptLanguage,
    distractor_type: DistractorType,
) -> Result<String, actix_web::Error> {
    match language {
        PromptLanguage::Urdu => Err(actix_web::error::ErrorBadRequest(
            "Urdu language is not supported for this endpoint",
        )),
        PromptLanguage::Arabic => Ok(
            utils::prompts::arabic_prompt_template_quranic_verse_distractor_mcq(
                question,
                correct_answer,
                distractor_type,
            ),
        ),
    }
}

pub async fn generate_mcq_options_from_context(
    req_body: web::Json<model::llm::ContextFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let language = utils::parse::map_to_prompt_language(&req_body.language);

    let prompt =
        build_contextual_mcq_prompt(&req_body.question, &req_body.correct_answer, language)?;

    let raw_output = send_prompt_to_llm(prompt, 1).await.map_err(|e| {
        error!("LLM API failure: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!("LLM API Error: {}", e))
    })?;

    let clean_text = utils::parse::clean_llm_json_output(&raw_output).map_err(|e| {
        error!("Failed to clean LLM output: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!("Cleaning error: {}", e))
    })?;

    let response: GuessFillInTheBlankResponse = serde_json::from_str(&clean_text).map_err(|e| {
        error!("Failed to parse MCQ options from cleaned text: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!("Parsing error: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(response))
}

pub async fn generate_quranic_verse_distractor_response<T>(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
    distractor_type: DistractorType,
) -> Result<HttpResponse, actix_web::Error>
where
    T: QuranDistractorResponse + Serialize,
{
    let prompt = get_quranic_verse_distractor_prompt(
        &req_body.question,
        &req_body.correct_answer,
        PromptLanguage::Arabic,
        distractor_type,
    )?;

    let raw_output = send_prompt_to_llm(prompt, 1).await.map_err(|e| {
        error!("LLM API failure: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!("LLM API error: {}", e))
    })?;

    let clean_text = utils::parse::clean_llm_json_output(&raw_output).map_err(|e| {
        error!("Failed to clean LLM output: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!("Cleaning error: {}", e))
    })?;

    let response: T = serde_json::from_str(&clean_text).map_err(|e| {
        error!("Failed to parse MCQ options from cleaned text: {:?}", e);
        actix_web::error::ErrorInternalServerError(format!("Parsing error: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(response))
}

pub async fn generate_collection(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<GuessFillInTheBlankQuranDistractorCollectionResponse>(
        req_body,
        DistractorType::Collection,
    ).await
}

pub async fn generate_morphological(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<MorphologicalDistractorResponse>(
        req_body,
        DistractorType::Morphological,
    )
    .await
}

pub async fn generate_diacritic(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<DiacriticDistractorResponse>(
        req_body,
        DistractorType::Diacritic,
    )
    .await
}

pub async fn generate_phonetic(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<PhoneticOrthographicDistractorResponse>(
        req_body,
        DistractorType::Phonetic,
    )
    .await
}

pub async fn generate_grammatical(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<GrammaticalDistractorResponse>(
        req_body,
        DistractorType::Grammatical,
    )
    .await
}

pub async fn generate_alternate_verse(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<AlternateVerseDistractorResponse>(
        req_body,
        DistractorType::AlternateVerse,
    )
    .await
}

pub async fn generate_thematic(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<ThematicDistractorResponse>(
        req_body,
        DistractorType::Thematic,
    )
    .await
}

pub async fn generate_collocational(
    req_body: web::Json<model::llm::QuranicVerseFillInThBlankTextGenerationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    generate_quranic_verse_distractor_response::<CollocationalDistractorResponse>(
        req_body,
        DistractorType::Collocational,
    )
    .await
}
