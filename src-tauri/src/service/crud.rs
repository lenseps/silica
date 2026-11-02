use core::fmt;
use std::{marker::PhantomData, path::PathBuf};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum CrudError {
    Io(std::io::Error),
    Serde(serde_json::Error),
    NotFound,
    DuplicateId,
}

impl fmt::Display for CrudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrudError::Io(e) => write!(f, "IO error: {e}"),
            CrudError::Serde(e) => write!(f, "JSON error: {e}"),
            CrudError::NotFound => write!(f, "Record not found."),
            CrudError::DuplicateId => write!(f, "Duplicated record with the same ID."),
        }
    }
}

impl std::error::Error for CrudError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CrudError::Io(e) => Some(e),
            CrudError::Serde(e) => Some(e),
            _ => None,
        }
    }
}
pub trait Persistable {
    type Id;
    fn get_id(&self) -> &Self::Id;
    fn load_meta_from(&mut self, old: &Self);
}
pub struct Crud<T> {
    path: PathBuf,
    mutex: tokio::sync::Mutex<()>,
    // PhantomData field. Crud does not store <T> at runtime.
    // Use PhantomData<fn() -> T> to keep Crud Sync + Send.
    _pd: PhantomData<fn() -> T>,
}

// TODO: Implement these methods in Crud.
impl<T> Crud<T>
where
    T: Persistable + Serialize + DeserializeOwned,
{
    pub fn new(path: impl Into<PathBuf>) -> Self {
        todo!();
    }
    pub async fn add(&self, model: T) -> Result<(), CrudError> {
        todo!()
    }
    pub async fn get_all(&self) -> Result<Vec<T>, CrudError> {
        todo!()
    }
    pub async fn get_by_id(&self, id: T::Id) -> Result<Option<T>, CrudError> {
        todo!()
    }
    pub async fn update(&self, id: T::Id, model: T) -> Result<(), CrudError> {
        todo!()
    }
    pub async fn delete_all(&self) -> Result<(), CrudError> {
        todo!()
    }
    pub async fn delete_by_id(&self, id: T::Id) -> Result<(), CrudError> {
        todo!()
    }
}
