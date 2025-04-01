use crate::{errors::AppError, utils};
use deadpool_redis::{Config, Pool};

#[derive(Clone)]
pub struct RedisClient {
    pub pool: Pool,
}

impl RedisClient {
    pub async fn new() -> Result<Self, AppError> {
        let redis_url = utils::env::load_env_var("REDIS_URL")?;

        let cfg = Config::from_url(redis_url);
        let pool = cfg
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .map_err(|err| AppError::CreatePoolError(err))?;

        log::info!("Successfully Connected to Redis with Deadpool");

        Ok(Self { pool })
    }

    pub async fn get_connection(&self) -> Result<deadpool_redis::Connection, AppError> {
        self.pool
            .get()
            .await
            .map_err(|err| AppError::PoolError(err))
    }
}

pub struct RedisSchema;

impl RedisSchema {
    pub fn exam_key(exam_id: &str) -> String {
        format!("exam:{}", exam_id)
    }

    pub fn question_key(exam_id: &str, question_id: &str) -> String {
        format!("exam:{}:question:{}", exam_id, question_id)
    }

    pub const FIELD_NAME: &'static str = "name";
    pub const FIELD_DETAILS: &'static str = "details";
    pub const FIELD_DURATION: &'static str = "duration";
    pub const FIELD_OPTIONS: &'static str = "options";
    pub const FIELD_CORRECT_OPTION: &'static str = "correct_option";
    pub const FIELD_MARKS: &'static str = "marks";
    pub const FIELD_SECTION: &'static str = "section";
    pub const FIELD_SECTION_ID: &'static str = "section_id";
    pub const FIELD_QUESTION_IDS: &'static str = "question_ids";
}
