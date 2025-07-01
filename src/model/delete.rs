use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteIdsRequest {
    pub section_ids: Vec<i32>,
    pub question_ids: Vec<i32>,
    pub option_ids: Vec<i32>,
}

impl DeleteIdsRequest {
    pub fn is_all_empty(&self) -> bool {
        self.section_ids.is_empty()
            && self.question_ids.is_empty()
            && self.option_ids.is_empty()
    }
}
