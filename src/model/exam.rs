use serde::{Serialize, Deserialize};
use crate::{database::schema, model::{delete::DeleteIdsRequest, section::SectionRequest, section::SectionResponse}};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamDescription {
    pub id: i32,
    pub exam_id: i32,
    pub title: String,
    pub description: String,
    pub duration: i32,
    pub passing_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamIdResponse {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct ExamIdResponseModel {
    #[serde(flatten)]
    pub base: schema::ExamModel,
}

#[derive(Serialize, Debug)]
pub struct ExamResponse {
    pub exam_id: ExamIdResponse,
    pub description: ExamDescription,
    pub sections: Vec<SectionResponse>,
}

// From impls
impl From<schema::ExamModel> for ExamIdResponse {
    fn from(model: schema::ExamModel) -> Self {
        ExamIdResponse { id: model.id }
    }
}

impl From<schema::ExamDescriptionModel> for ExamDescription {
    fn from(model: schema::ExamDescriptionModel) -> Self {
        ExamDescription {
            id: model.id,
            exam_id: model.exam_id,
            title: model.title,
            description: model.description.unwrap_or_default(),
            duration: model.duration,
            passing_score: model.passing_score,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ExamIdRequestModel {
    #[serde(flatten)]
    pub base: schema::ExamModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamDescriptionRequest {
    #[serde(flatten)]
    pub base: schema::ExamDescriptionModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamRequest {
    pub exam_id: ExamIdRequestModel,
    pub description: ExamDescriptionRequest,
    pub sections: Vec<SectionRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditExamRequest {
    pub exam_id: ExamIdRequestModel,
    pub description: ExamDescriptionRequest,
    pub sections: Vec<SectionRequest>,
    pub delete: DeleteIdsRequest,
}
