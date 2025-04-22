use crate::db;

#[derive(Clone)]
pub struct AppState {
    pub db_client: db::DbClient,
}
