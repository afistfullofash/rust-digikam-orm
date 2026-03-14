use diesel::prelude::*;
use thiserror;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum DigikamDatabaseError {
    #[error("Could not connect to the digiKam Database at path: {0}")]
    ConnectionFailed(String),
}

pub fn get_connection(database_path: String) -> Result<SqliteConnection, DigikamDatabaseError> {
    match SqliteConnection::establish(&database_path) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!(error = ?e, "Error connecting to database");
            Err(DigikamDatabaseError::ConnectionFailed(
                database_path.clone(),
            ))
        }
    }
}
