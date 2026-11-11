use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    UnknownError(String),
    CrudError(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::UnknownError(error) => write!(f, "Unknown Error: {}", error),
            CommandError::CrudError(error) => write!(f, "CRUD Error: {}", error),
        }
    }
}
