use diesel::result::Error as DieselError;
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Deserialize)]
pub struct SQLError {
    pub message: String,
}

impl SQLError {
    pub fn new(message: String) -> Self {
        SQLError { message }
    }
}

impl From<DieselError> for SQLError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => SQLError::new(err.message().to_string()),
            DieselError::NotFound => SQLError::new("Record not found".to_string()),
            err => SQLError::new(format!("Diesel error: {}", err)),
        }
    }
}

impl Display for SQLError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str(self.message.as_str())
    }
}
