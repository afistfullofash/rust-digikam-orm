use diesel::prelude::*;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum DigikamDatabaseError {
    #[error("Could not connect to the digiKam Database at path: {0}")]
    ConnectionFailed(&str),
}

pub fn get_connection(database_path: &str) -> Result<SqliteConnection, DigikamDatabaseError> {
    match SqliteConnection::establish(&database_path) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!(error = ?e, "Error connecting to database");
            Err(DigikamDatabaseError::ConnectionFailed(database_path))
        }
    }
}
