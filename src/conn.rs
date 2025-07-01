use crate::utils;
use anyhow::{Context, Result};
use log::info;
use sqlx::{postgres::PgPoolOptions, PgPool};

/// A struct representing the database client connection.
///
/// This struct holds a reference to a `DatabaseConnection` that is used to interact with the database.
/// It is intended to be cloned to allow passing it around within the application.
///
/// # Fields
///
/// * `db` - The actual database connection instance, allowing database queries to be executed.
///
/// # Example
///
/// ```rust
/// let db_client = DbClient::new().await?;
/// ```
#[derive(Clone)]
pub struct DbClient {
    pub pool: PgPool,
}

impl DbClient {
    /// Creates a new instance of `DbClient` by connecting to the database.
    ///
    /// This function attempts to load the database URL from an environment variable and
    /// connects to the database asynchronously. If the connection is successful, it
    /// returns an instance of `DbClient` containing the `DatabaseConnection`.
    /// If an error occurs during the connection, it returns an `AppError::DbErr`.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self, AppError>`. If the connection is successful, it returns
    /// `Ok(DbClient)` containing the connection. Otherwise, it returns an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// let db_client = DbClient::new().await?;
    /// ```
    pub async fn new() -> Result<Self> {
        let db_url = utils::env::load_env_var("DATABASE_URL")
            .context("Failed to load DATABASE_URL environment variable")?; // Load database URL from env variable

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .context("Failed to connect to the PostgreSQL database")?;

        info!("Successfully Connected to DB"); // Log the success message
        Ok(DbClient { pool }) // Return the DbClient with the connection
    }

    /// Runs database migrations using SQLx.
    ///
    /// This function applies all pending migrations from the `migrations` directory.
    /// It returns an error if migrations fail.
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run database migrations")?;
        info!("Database migrations applied successfully");
        Ok(())
    }
}


#[derive(Clone)]
pub struct UrlBuilder {
    base_url: String,
    model_name: String,
    api_key: String,
}

impl UrlBuilder {
    pub fn new() -> Result<Self> {
        let base_url = utils::env::load_env_var("TEXT_GENERATION_URL")
            .context("Failed to load TEXT_GENERATION_URL environment variable")?;

        let model_name = utils::env::load_env_var("TEXT_GENERATION_MODEL")
            .context("Failed to load TEXT_GENERATION_MODEL environment variable")?;

        let api_key = utils::env::load_env_var("TEXT_GENERATION_API_KEY")
            .context("Failed to load TEXT_GENERATION_API_KEY environment variable")?;

        Ok(Self {
            base_url,
            model_name,
            api_key,
        })
    }

    pub fn get_url(&self) -> String {
        format!("{}/{}:generateContent?key={}", self.base_url, self.model_name, self.api_key)
    }

}