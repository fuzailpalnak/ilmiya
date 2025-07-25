use crate::database::schema;
use crate::model;
use crate::model::llm::PromptLanguage;
use crate::model::option::OptionResponseModel;
use crate::model::question::QuestionResponse;
use crate::model::section::SectionResponse;
use anyhow::Result;
use std::collections::HashMap;

/// Maps the raw rows from the database query to structured `SectionResponse` objects.
/// Returns a Result with either the mapped data or an error.
pub fn map_to_section_response(
    rows: Vec<schema::SectionRow>,
) -> Result<HashMap<i32, SectionResponse>> {
    let mut sections_map = HashMap::new();

    for row in rows {
        let section_id = row.section_id;

        let section_model = schema::SectionsModel {
            id: row.section_id,
            exam_description_id: row.section_exam_description_id,
            title: row.section_title.clone(),
        };

        let section = sections_map
            .entry(section_id)
            .or_insert_with(|| SectionResponse {
                base: section_model,
                questions: Vec::new(),
            });

        // Build the question base model
        let question_model = schema::QuestionsModel {
            id: row.question_id,
            section_id: row.section_id,
            text: row.question_text.clone(),
            description: row.question_description.clone(),
            marks: row.question_marks,
        };

        let question_id = question_model.id;

        if let Some(existing_question) = section
            .questions
            .iter_mut()
            .find(|q| q.base.id == question_id)
        {
            let option_model = schema::OptionsModel {
                id: row.option_id,
                question_id,
                text: row.option_text.clone(),
                is_correct: row.option_is_correct,
            };

            existing_question
                .options
                .push(OptionResponseModel { base: option_model });
        } else {
            let mut question = QuestionResponse {
                base: question_model,
                options: Vec::new(),
            };

            let option_model = schema::OptionsModel {
                id: row.option_id,
                question_id,
                text: row.option_text.clone(),
                is_correct: row.option_is_correct,
            };

            question
                .options
                .push(OptionResponseModel { base: option_model });

            section.questions.push(question);
        }
    }

    // Return the map of sections or an error if anything goes wrong
    Ok(sections_map)
}

pub fn map_to_prompt_language(language: &model::llm::Language) -> PromptLanguage {
    match language {
        model::llm::Language::Arabic => PromptLanguage::Arabic,
        model::llm::Language::Urdu => PromptLanguage::Urdu,
    }
}

pub fn clean_llm_json_output(json_text: &str) -> Result<String, anyhow::Error> {
    let mut clean_text = json_text.trim();

    // Remove the code block markers and possible 'json' after the first ```
    if clean_text.starts_with("```") {
        clean_text = clean_text.trim_start_matches("```").trim_start();

        if clean_text.starts_with("json") {
            clean_text = clean_text.trim_start_matches("json").trim_start();
        }

        if clean_text.ends_with("```") {
            clean_text = clean_text.trim_end_matches("```").trim();
        }
    }

    Ok(clean_text.to_string())
}
