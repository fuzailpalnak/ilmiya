use crate::utils;
use anyhow::{Context, Result};
use deadpool_redis::{Config, Pool, Runtime};
use log::info;
use sqlx::{postgres::PgPoolOptions, PgPool};

/// A struct representing the Redis client connection.
///
/// This struct wraps a connection pool to Redis and allows easy access
/// to a Redis connection.
///
/// # Fields
///
/// * `pool` - The Redis connection pool.
///
#[derive(Clone)]
pub struct RedisClient {
    pub pool: Pool,
}

impl RedisClient {
    /// Creates a new instance of `RedisClient` by connecting to the Redis server.
    ///
    /// Loads the Redis URL from an environment variable and initializes the connection pool.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self>`. If the connection is successful, it returns `Ok(RedisClient)`.
    /// Otherwise, it returns an error.
    pub async fn new() -> Result<Self> {
        let redis_url = utils::env::load_env_var("REDIS_URL")
            .context("Failed to load REDIS_URL environment variable")?;

        let cfg = Config::from_url(redis_url);
        let pool = cfg
            .create_pool(Some(Runtime::Tokio1))
            .context("Failed to create Redis connection pool")?;

        info!("Successfully connected to Redis");
        Ok(Self { pool })
    }

    /// Gets a Redis connection from the pool.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Connection>`, which is a pooled Redis connection ready to use.
    pub async fn get_connection(&self) -> Result<deadpool_redis::Connection> {
        self.pool
            .get()
            .await
            .context("Failed to get Redis connection from the pool")
    }
}

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
