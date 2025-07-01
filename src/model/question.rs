use serde::{Serialize, Deserialize};
use crate::{database::schema, model::option::{OptionRequestModel, OptionResponseModel}};

use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct QuestionResponse {
    #[serde(flatten)]
    pub base: schema::QuestionsModel,
    pub options: Vec<OptionResponseModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionRequest {
    #[serde(flatten)]
    pub base: schema::QuestionsModel,
    pub options: Vec<OptionRequestModel>,
}
