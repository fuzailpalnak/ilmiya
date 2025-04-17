use crate::{errors::AppError, utils};
use log::info;
use sea_orm::sea_query::{Alias, IntoIden, SelectExpr, SelectStatement};
use sea_orm::Iden;
use sea_orm::{ColumnTrait, EntityTrait, QueryTrait};
use sea_orm::{Database, DatabaseConnection};

/// Prefixer utility to prefix selected column names from entities
/// https://github.com/SeaQL/sea-orm/discussions/1502
pub struct Prefixer<S: QueryTrait<QueryStatement = SelectStatement>> {
    pub selector: S,
}

impl<S: QueryTrait<QueryStatement = SelectStatement>> Prefixer<S> {
    pub fn new(selector: S) -> Self {
        Self { selector }
    }
    pub fn add_columns<T: EntityTrait>(self, entity: T) -> Self
    where
        T::Column: Copy + sea_orm::entity::Iterable,
    {
        let columns: Vec<T::Column> = <T::Column as sea_orm::entity::Iterable>::iter().collect();
        self.add_columns_inner(entity, &columns)
    }

    pub fn add_columns_from_list<T: EntityTrait>(self, entity: T, columns: &[T::Column]) -> Self
    where
        T::Column: Copy,
    {
        self.add_columns_inner(entity, columns)
    }

    fn add_columns_inner<T: EntityTrait>(mut self, entity: T, columns: &[T::Column]) -> Self
    where
        T::Column: Copy,
    {
        for &col in columns {
            let alias = format!("{}{}", entity.table_name(), col.to_string());
            self.selector.query().expr(SelectExpr {
                expr: col.select_as(col.into_expr()),
                alias: Some(Alias::new(&alias).into_iden()),
                window: None,
            });
        }
        self
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
