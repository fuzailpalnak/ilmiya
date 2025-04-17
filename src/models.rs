use std::collections::HashSet;

use crate::db;
use crate::entities::{details, options, questions, sections};
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub db_client: db::DbClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamDescription {
    pub title: String,
    pub description: String,
    pub duration: i32,
    pub passing_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub section_id: i32,
    pub text: String,
    pub description: String,
    pub marks: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionModel {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamId {
    pub exam_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exam {
    pub exam_description: ExamDescription,
    pub sections: Vec<Section>,
    pub questions: Vec<Question>,
    pub options: Vec<OptionModel>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Delete {
    pub section_ids: Vec<i32>,
    pub question_ids: Vec<i32>,
    pub option_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditExam {
    pub exam_id: ExamId,
    pub exam_description: ExamDescription,
    pub sections: Vec<Section>,
    pub questions: Vec<Question>,
    pub options: Vec<OptionModel>,
    pub delete: Delete,
}

#[derive(Debug)]
pub struct DeletionData {
    pub section_id_to_delete: HashSet<i32>,
    pub question_id_to_delete: HashSet<i32>,
    pub option_id_to_delete: HashSet<i32>,
    pub section_question_option_map: Vec<(i32, i32, i32)>,
}

#[derive(Debug)]
pub struct QueryResponse {
    pub exam: details::Model,
    pub section: sections::Model,
    pub question: questions::Model,
    pub option: options::Model,
}

impl FromQueryResult for QueryResponse {
    fn from_query_result(res: &sea_orm::QueryResult, _pre: &str) -> Result<Self, sea_orm::DbErr> {
        let exam = parse_query_to_model::<details::Model, details::Entity>(res)?.into();
        let section = parse_query_to_model::<sections::Model, sections::Entity>(res)?;
        let question = parse_query_to_model::<questions::Model, questions::Entity>(res)?;
        let option = parse_query_to_model::<options::Model, options::Entity>(res)?;

        Ok(Self {
            exam,
            section,
            question,
            option,
        })
    }
}

pub fn parse_query_to_model<M: FromQueryResult, E: EntityTrait>(
    res: &sea_orm::QueryResult,
) -> Result<M, sea_orm::DbErr> {
    M::from_query_result(res, E::default().table_name())
}

impl Delete {
    pub fn is_all_empty(&self) -> bool {
        self.section_ids.is_empty() && self.question_ids.is_empty() && self.option_ids.is_empty()
    }
}

impl DeletionData {
    pub fn new(
        section_id_to_delete: Vec<i32>,
        question_id_to_delete: Vec<i32>,
        option_id_to_delete: Vec<i32>,
        section_question_option_map: Vec<(i32, i32, i32)>,
    ) -> Self {
        Self {
            section_id_to_delete: section_id_to_delete.into_iter().collect(),
            question_id_to_delete: question_id_to_delete.into_iter().collect(),
            option_id_to_delete: option_id_to_delete.into_iter().collect(),
            section_question_option_map,
        }
    }

    pub fn filter_deletions(&mut self) {
        for (section_id, question_id, option_id) in &self.section_question_option_map {
            if self.section_id_to_delete.contains(section_id) {
                if !self.question_id_to_delete.contains(question_id) {
                    self.question_id_to_delete.insert(*question_id);
                }
                if !self.option_id_to_delete.contains(option_id) {
                    self.option_id_to_delete.insert(*option_id);
                }
            }
        }
    }
}
