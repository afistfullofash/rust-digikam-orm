//! Database Utilities
//! General Functions and types for interacting with the database
use diesel::prelude::*;
use tracing::error;

/// Signal a Error when Connecting to the Database
#[derive(thiserror::Error, Debug)]
pub enum DigikamDatabaseError {
    #[error("Could not connect to the digiKam Database at path: {0}")]
    ConnectionFailed(String),
}

/// Get a connection to the digiKam Database
///
/// # Arguments
/// * `database_path` - The path to the digiKam Sqlite Database file
pub fn get_connection(database_path: &str) -> Result<SqliteConnection, DigikamDatabaseError> {
    match SqliteConnection::establish(database_path) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!(error = ?e, "Error connecting to database");
            Err(DigikamDatabaseError::ConnectionFailed(
                database_path.to_string(),
            ))
        }
    }
}
