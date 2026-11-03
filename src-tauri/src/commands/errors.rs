use core::fmt;

use serde::{Deserialize, Serialize};

use crate::service::crud::CrudError;

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    InvalidInput(String),
    CrudError(CrudError),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::InvalidInput(error) => write!(f, "Invalid input: {}", error),
            CommandError::CrudError(error) => write!(f, "CRUD Error: {}", error),
        }
    }
}
