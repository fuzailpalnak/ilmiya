use crate::conn;

#[derive(Clone)]
pub struct AppState {
    pub db_client: conn::DbClient,
}
