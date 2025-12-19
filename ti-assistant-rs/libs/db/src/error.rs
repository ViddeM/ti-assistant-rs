use diesel::ConnectionError;
use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};

use crate::game_id::GameId;

/// Result for the DB crate.
pub type DbResult<T> = Result<T, DbError>;

/// Errors that can occur in the DB crate.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("DB Pool error: {0}")]
    PoolError(#[from] PoolError),
    #[error("Failed to build DB Pool: {0}")]
    BuildPoolError(#[from] BuildError),
    #[error("Diesel query error: {0}")]
    QueryError(#[from] diesel::result::Error),
    #[error("Connection error: {0}")]
    ConnectionError(#[from] ConnectionError),
    #[error("Failed to delete game event for game_id: {game_id} due to error: {error}")]
    DeleteGameError { game_id: GameId, error: String },
    #[error("Failed to run migrations: {0}")]
    MigrationError(String),
}
