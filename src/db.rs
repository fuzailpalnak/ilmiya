use crate::{errors::AppError, utils};
use log::info;
use sea_orm::{Database, DatabaseConnection};

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
    pub db: DatabaseConnection,
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
    pub async fn new() -> Result<Self, AppError> {
        let db_url = utils::env::load_env_var("DATABASE_URL")?; // Load database URL from env variable
        let db = Database::connect(&db_url)
            .await
            .map_err(|err| AppError::DbErr(err))?; // Attempt to connect to the database
        info!("Successfully Connected to DB"); // Log the success message
        Ok(DbClient { db }) // Return the DbClient with the connection
    }
}
