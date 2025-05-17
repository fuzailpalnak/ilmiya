use crate::conn;

#[derive(Clone)]
pub struct AppState {
    pub db_client: conn::DbClient,
    pub text_generation_api: conn::UrlBuilder
}
