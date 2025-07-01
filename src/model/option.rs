use serde::{Serialize, Deserialize};
use crate::database::schema;


#[derive(Debug, Serialize)]
pub struct OptionResponseModel {
    #[serde(flatten)]
    pub base: schema::OptionsModel,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OptionRequestModel {
    #[serde(flatten)]
    pub base: schema::OptionsModel,
}
