use crate::database::schema;
use crate::model::response::{OptionResponseModel, QuestionResponse, SectionResponse};
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
            details_id: row.section_details_id,
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
