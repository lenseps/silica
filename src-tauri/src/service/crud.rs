use core::fmt;
use std::{
    any::TypeId,
    collections::BTreeSet,
    fmt::Display,
    marker::PhantomData,
    path::PathBuf,
    sync::Mutex,
};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, sync::MutexGuard};

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

/*Convertation Error write/out, and realize trait for CrudError*/
impl From<serde_json::Error> for CrudError {
    fn from(err: serde_json::Error) -> Self {
        CrudError::Serde(err)
    }
}

impl From<std::io::Error> for CrudError {
    fn from(err: std::io::Error) -> Self {
        CrudError::Io(err)
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

//A collection for storing unique types
static ID_SET: Mutex<BTreeSet<TypeId>> = Mutex::new(BTreeSet::new());

// TODO: Implement these methods in Crud.
impl<T: Display + 'static> Crud<T>
where
    T: Persistable + Serialize + DeserializeOwned,
    T::Id: PartialEq,
{
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let mut set = ID_SET.lock().unwrap();
        set.insert(TypeId::of::<T>());

        Self {
            path: path.into(),
            mutex: tokio::sync::Mutex::new(()),
            _pd: PhantomData,
        }
    }

    //Auxiliary method for reading
    async fn read(&self) -> Result<Vec<T>, CrudError> {
        // opening a file asynchronously
        let mut file = match tokio::fs::File::open(&self.path).await {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()), // if file not found, return empty
            Err(e) => return Err(CrudError::Io(e)),
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?; // Read everything into the buffer

        if buffer.is_empty() {
            return Ok(Vec::new());
        }

        // Deserialize bytes in Vec<T>
        let items = serde_json::from_slice(&buffer)?;
        Ok(items)
    }

    async fn write(&self, items: &[T]) -> Result<(), CrudError> {
        let bytes = serde_json::to_vec_pretty(items)?;
        
        let mut file = tokio::fs::File::create(&self.path).await?;
        file.write_all(&bytes).await?;
        Ok(())
    }

    pub async fn add(&self, model: T) -> Result<(), CrudError> {
        let _guard = self.mutex.lock().await; // we are block duration of the operation

        let mut items = self.read().await?;

        if items.iter().any(|i| i.get_id() == model.get_id()) {
             return Err(CrudError::DuplicateId);
        }

        items.push(model);
        self.write(&items).await?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<T>, CrudError> {
        let _guard = self.mutex.lock().await;
        let items = self.read().await?;
        Ok(items) 
    }

    pub async fn get_by_id(&self, id: T::Id) -> Result<Option<T>, CrudError> {
        let _guard = self.mutex.lock().await;
        let items = self.read().await?;
        Ok(items.into_iter().find(|t| *t.get_id() == id))
    }

    pub async fn update(&self, id: T::Id, model: T) -> Result<(), CrudError> {
        let _guard = self.mutex.lock().await;
        let mut items = self.read().await?;
        
        let pos = items
            .iter()
            .position(|t| *t.get_id() == id)
            .ok_or(CrudError::NotFound)?;
            
        let mut model = model;
        
        //checked model
        if model.get_id() != &id {
            return Err(CrudError::NotFound);
        }
        model.load_meta_from(&items[pos]);
        items[pos] = model;
        
        self.write(&items).await?;
        Ok(())
    }

    pub async fn delete_all(&self) -> Result<(), CrudError> {
        let _guard = self.mutex.lock().await;

        if !tokio::fs::try_exists(&self.path).await? {
            return Ok(());
        }

        self.write(&Vec::new()).await?;

        Ok(())
    }

    pub async fn delete_by_id(&self, id: T::Id) -> Result<(), CrudError> {
        let _guard = self.mutex.lock().await;
        let mut items = self.read().await?;
        let len = items.len();
        
        items.retain(|t| *t.get_id() != id);
        
        if items.len() == len {
            return Err(CrudError::NotFound);
        }
        
        self.write(&items).await?;
        Ok(())
    }
}
