use actix_web::{HttpResponse, ResponseError};
use log::error;
use sea_orm::DbErr;
use serde_json::Error as SerdeError;
use std::{fmt, io::Error as IOError, time::SystemTimeError};

/// A custom error enum used to handle different types of errors in the application.
///
/// This enum is used to represent various error types that can occur during the operation
/// of the application. It includes errors related to HTTP handling, input/output, database
/// interactions, system time, and serialization.
///
/// # Variants
///
/// * `ActixError` - Represents an error that occurs while handling HTTP requests with Actix Web.
/// * `IOError` - Represents I/O-related errors (e.g., file operations, network operations).
/// * `NotFound` - Represents a "resource not found" error, typically returned when a resource
///   is not found in the database or API.
/// * `SystemTimeError` - Represents errors that occur while working with system time.
/// * `DbErr` - Represents errors related to database operations (e.g., database connection, query errors).
/// * `SerdeError` - Represents errors related to serialization or deserialization (using Serde).
///
/// # Example
///
/// ```rust
/// let err = AppError::NotFound("Item not found".into());
/// ```
#[derive(Debug)]
pub enum AppError {
    ActixError(actix_web::Error),
    IOError(IOError),
    NotFound(String),
    SystemTimeError(SystemTimeError),
    DbErr(DbErr),
    SerdeError(SerdeError),
}

impl fmt::Display for AppError {
    /// Custom implementation of `fmt::Display` for the `AppError` enum.
    ///
    /// This implementation formats the error into a human-readable string representation. It
    /// uses pattern matching to handle each variant of the enum and display an appropriate
    /// message for each error type.
    ///
    /// # Example
    ///
    /// ```rust
    /// let err = AppError::NotFound("Item not found".into());
    /// println!("{}", err);  // Prints: "Resource not found: Item not found"
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ActixError(e) => write!(f, "Actix error: {}", e),
            AppError::IOError(e) => write!(f, "I/O error: {}", e),
            AppError::NotFound(msg) => write!(f, "Resource not found: {}", msg),
            AppError::SystemTimeError(e) => write!(f, "System time error: {}", e),
            AppError::DbErr(e) => write!(f, "DbErr error: {}", e),
            AppError::SerdeError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl ResponseError for AppError {
    /// Custom implementation of the `ResponseError` trait for `AppError`.
    ///
    /// This implementation allows `AppError` to be used as an Actix Web error response.
    /// It generates the corresponding HTTP response for each error variant, including
    /// `InternalServerError`, `NotFound`, and `BadRequest`, with a JSON body containing
    /// an error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// let err = AppError::NotFound("Item not found".into());
    /// let response = err.error_response();  // Returns a NotFound response with error details.
    /// ```
    fn error_response(&self) -> HttpResponse {
        error!("Error occurred: {}", self);

        match self {
            AppError::ActixError(_)
            | AppError::IOError(_)
            | AppError::DbErr(_)
            | AppError::SerdeError(_) => HttpResponse::InternalServerError().json({
                serde_json::json!({"error": "Internal Server Error", "message": self.to_string()})
            }),
            AppError::NotFound(_) => HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Not Found", "message": self.to_string()})),
            AppError::SystemTimeError(_) => HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Bad Request", "message": self.to_string()})),
        }
    }
}

// Implement `From` trait for various error types to easily convert them into `AppError`.

impl From<SerdeError> for AppError {
    fn from(e: SerdeError) -> Self {
        AppError::SerdeError(e)
    }
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        AppError::DbErr(e)
    }
}

impl From<SystemTimeError> for AppError {
    fn from(e: SystemTimeError) -> Self {
        AppError::SystemTimeError(e)
    }
}

impl From<actix_web::Error> for AppError {
    fn from(e: actix_web::Error) -> Self {
        AppError::ActixError(e)
    }
}

impl From<IOError> for AppError {
    fn from(e: IOError) -> Self {
        AppError::IOError(e)
    }
}
