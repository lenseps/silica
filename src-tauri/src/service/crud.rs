use core::fmt;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{marker::PhantomData, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum CrudError {
    Io(String),
    Serde(String),
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

/*Convertation Error write/out, and realize trait for CrudError*/
impl From<serde_json::Error> for CrudError {
    fn from(err: serde_json::Error) -> Self {
        CrudError::Serde(err.to_string())
    }
}

impl From<std::io::Error> for CrudError {
    fn from(err: std::io::Error) -> Self {
        CrudError::Io(err.to_string())
    }
}

pub trait Persistable {
    type Id;
    fn get_id(&self) -> &Self::Id;
    fn load_meta_from(&mut self, old: &Self);
}

#[derive(Debug)]
pub struct Crud<T> {
    path: PathBuf,
    mutex: tokio::sync::Mutex<()>,
    // PhantomData field. Crud does not store <T> at runtime.
    // Use PhantomData<fn() -> T> to keep Crud Sync + Send.
    _pd: PhantomData<fn() -> T>,
}

// TODO: Add tests to check all the `Crud` functional.
impl<T> Crud<T>
where
    T: Persistable + Serialize + DeserializeOwned,
    T::Id: PartialEq,
{
    pub fn new(path: impl Into<PathBuf>) -> Self {
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
            Err(e) => return Err(CrudError::Io(e.to_string())),
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

        let mut tmp = self.path.clone().into_os_string();
        tmp.push(format!("{}.tmp", Uuid::new_v4()));
        let tmp = PathBuf::from(tmp);

        {
            let mut f = tokio::fs::File::create(&tmp).await?;
            f.write_all(&bytes).await?;
            f.sync_all().await?;
        }
        tokio::fs::rename(&tmp, &self.path).await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeSet;
    use std::{path::PathBuf, sync::Arc};
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestModel {
        id: u32,
        name: String,
        created: String,
    }

    impl Persistable for TestModel {
        type Id = u32;

        fn get_id(&self) -> &Self::Id {
            &self.id
        }

        fn load_meta_from(&mut self, old: &Self) {
            self.id = old.id;
            self.created = old.created.clone();
        }
    }
    /*
     Each test receives its own unique file.
     Therefore, running tests in parallel does not cause conflicts.
    */
    fn test_path() -> PathBuf {
        std::env::temp_dir().join(format!("crud_test_{}.json", Uuid::new_v4()))
    }

    fn model(id: u32, name: &str) -> TestModel {
        TestModel {
            id,
            name: name.to_string(),
            created: "2026-01-01".to_string(),
        }
    }

    async fn cleanup(path: &PathBuf) {
        let _ = tokio::fs::remove_file(path).await;
    }

    //Private method

    #[tokio::test]
    async fn read_returns_empty_vec_when_file_does_not_exist() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        let result = crud.read().await.unwrap();

        assert!(result.is_empty());

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn write_creates_valid_json_file() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        let items = vec![model(1, "First"), model(2, "Second")];

        crud.write(&items).await.unwrap();

        assert!(tokio::fs::try_exists(&path).await.unwrap());

        // We check that the file actually contains valid JSON.
        let content = tokio::fs::read(&path).await.unwrap();
        let from_file: Vec<TestModel> = serde_json::from_slice(&content).unwrap();

        assert_eq!(from_file, items);

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn read_returns_data_written_to_file() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        let expected = vec![model(1, "First"), model(2, "Second")];

        crud.write(&expected).await.unwrap();

        let result = crud.read().await.unwrap();

        assert_eq!(result, expected);

        cleanup(&path).await;
    }

    // Add
    #[tokio::test]
    async fn add_appends_model_and_preserves_order() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();
        crud.add(model(2, "Second")).await.unwrap();
        crud.add(model(3, "Third")).await.unwrap();

        // Checking the connection add -> get_all.
        let items = crud.get_all().await.unwrap();

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].id, 1);
        assert_eq!(items[1].id, 2);
        assert_eq!(items[2].id, 3);

        let content = tokio::fs::read(&path).await.unwrap();
        let file_items: Vec<TestModel> = serde_json::from_slice(&content).unwrap();

        assert_eq!(file_items, items);

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn add_returns_duplicate_id_error() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();

        let result = crud.add(model(1, "Duplicate")).await;

        assert!(matches!(result, Err(CrudError::DuplicateId)));

        cleanup(&path).await;
    }

    // get all
    #[tokio::test]
    async fn get_all_returns_empty_vec_for_empty_storage() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        let items = crud.get_all().await.unwrap();

        assert!(items.is_empty());

        cleanup(&path).await;
    }

    // get by id
    #[tokio::test]
    async fn get_by_id_returns_correct_model() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();
        crud.add(model(2, "Second")).await.unwrap();

        let result = crud.get_by_id(2).await.unwrap();

        assert_eq!(result, Some(model(2, "Second")));

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn get_by_id_returns_none_when_model_not_found() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();

        let result = crud.get_by_id(999).await.unwrap();

        assert_eq!(result, None);

        cleanup(&path).await;
    }

    // update
    #[tokio::test]
    async fn update_changes_model_and_preserves_metadata() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        let original = TestModel {
            id: 1,
            name: "Old name".to_string(),
            created: "old-date".to_string(),
        };

        crud.add(original).await.unwrap();

        let updated = TestModel {
            id: 1,
            name: "New name".to_string(),
            created: "new-date".to_string(),
        };

        crud.update(1, updated).await.unwrap();

        let result = crud.get_by_id(1).await.unwrap().unwrap();

        assert_eq!(result.id, 1);
        assert_eq!(result.name, "New name");

        assert_eq!(result.created, "old-date");

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn update_returns_not_found_when_id_does_not_exist() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        let result = crud.update(1, model(1, "New name")).await;

        assert!(matches!(result, Err(CrudError::NotFound)));

        cleanup(&path).await;
    }

    // delete all
    #[tokio::test]
    async fn delete_all_clears_data_but_keeps_file() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();
        crud.add(model(2, "Second")).await.unwrap();

        crud.delete_all().await.unwrap();

        assert!(tokio::fs::try_exists(&path).await.unwrap());

        let items = crud.get_all().await.unwrap();
        assert!(items.is_empty());

        let content = tokio::fs::read_to_string(&path).await.unwrap();
        assert_eq!(content.trim(), "[]");

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn delete_all_is_no_op_when_file_does_not_exist() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        let result = crud.delete_all().await;

        assert!(result.is_ok());
        assert!(!tokio::fs::try_exists(&path).await.unwrap());
    }

    // delete by id
    #[tokio::test]
    async fn delete_by_id_removes_only_requested_model() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();
        crud.add(model(2, "Second")).await.unwrap();
        crud.add(model(3, "Third")).await.unwrap();

        crud.delete_by_id(2).await.unwrap();

        let items = crud.get_all().await.unwrap();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, 1);
        assert_eq!(items[1].id, 3);

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn delete_by_id_returns_not_found_when_id_does_not_exist() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();

        let result = crud.delete_by_id(999).await;

        assert!(matches!(result, Err(CrudError::NotFound)));

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn crud_full_lifecycle() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        // add
        crud.add(model(1, "First")).await.unwrap();
        crud.add(model(2, "Second")).await.unwrap();

        // get_all
        assert_eq!(crud.get_all().await.unwrap().len(), 2);

        // get_by_id
        let item = crud.get_by_id(1).await.unwrap().unwrap();
        assert_eq!(item.name, "First");

        // update
        let updated = TestModel {
            id: 1,
            name: "Updated".to_string(),
            created: "another-date".to_string(),
        };

        crud.update(1, updated).await.unwrap();

        let item = crud.get_by_id(1).await.unwrap().unwrap();
        assert_eq!(item.name, "Updated");

        // delete_by_id
        crud.delete_by_id(2).await.unwrap();
        assert_eq!(crud.get_all().await.unwrap().len(), 1);

        // delete_all
        crud.delete_all().await.unwrap();
        assert!(crud.get_all().await.unwrap().is_empty());

        cleanup(&path).await;
    }
    #[tokio::test]
    async fn update_returns_error_when_model_id_differs_from_requested_id() {
        let path = test_path();
        let crud = Crud::<TestModel>::new(&path);

        crud.add(model(1, "First")).await.unwrap();

        let result = crud.update(1, model(2, "Different ID")).await;

        assert!(matches!(result, Err(CrudError::NotFound)));

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn concurrent_adds_through_same_crud_store_all_records() {
        let path = test_path();
        let crud = Arc::new(Crud::<TestModel>::new(&path));

        const N: u32 = 25;

        let mut handles = Vec::with_capacity(N as usize);
        for i in 0..N {
            let crud = crud.clone();
            handles.push(tokio::spawn(async move {
                crud.add(model(i, &format!("item-{i}"))).await
            }));
        }

        for h in handles {
            h.await.unwrap().unwrap();
        }

        let items = crud.get_all().await.unwrap();
        assert_eq!(items.len() as u32, N);

        // All the ids are unique.
        let ids: BTreeSet<u32> = items.iter().map(|m| m.id).collect();
        assert_eq!(ids.len() as u32, N);

        let content = tokio::fs::read(&path).await.unwrap();
        let file_items: Vec<TestModel> = serde_json::from_slice(&content).unwrap();
        assert_eq!(file_items, items);

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn concurrent_duplicate_adds_only_one_succeeds() {
        let path = test_path();
        let crud = Arc::new(Crud::<TestModel>::new(&path));

        const N: usize = 10;

        let mut handles = Vec::with_capacity(N);
        for i in 0..N {
            let crud = crud.clone();
            handles.push(tokio::spawn(async move {
                crud.add(model(1, &format!("attempt-{i}"))).await
            }));
        }

        let mut successes = 0usize;
        let mut duplicates = 0usize;
        for h in handles {
            match h.await.unwrap() {
                Ok(()) => successes += 1,
                Err(CrudError::DuplicateId) => duplicates += 1,
                Err(e) => panic!("unexpected error: {e:?}"),
            }
        }

        assert_eq!(successes, 1);
        assert_eq!(duplicates, N - 1);

        let items = crud.get_all().await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, 1);

        cleanup(&path).await;
    }

    #[tokio::test]
    async fn concurrent_adds_and_reads_always_see_valid_data() {
        let path = test_path();
        let crud = Arc::new(Crud::<TestModel>::new(&path));

        // Seed
        for i in 0..10u32 {
            crud.add(model(i, &format!("seed-{i}"))).await.unwrap();
        }

        let mut writers = Vec::new();
        for i in 10..50u32 {
            let crud = crud.clone();
            writers.push(tokio::spawn(async move {
                crud.add(model(i, &format!("item-{i}"))).await
            }));
        }

        let mut readers = Vec::new();
        for _ in 0..20 {
            let crud = crud.clone();
            readers.push(tokio::spawn(async move {
                let items = crud.get_all().await?;
                for item in &items {
                    assert!(
                        item.name.starts_with("seed-") || item.name.starts_with("item-"),
                        "unexpected name: {}",
                        item.name
                    );
                }
                Ok::<(), CrudError>(())
            }));
        }

        for h in writers {
            h.await.unwrap().unwrap();
        }
        for h in readers {
            h.await.unwrap().unwrap();
        }

        let items = crud.get_all().await.unwrap();
        assert_eq!(items.len(), 50);

        cleanup(&path).await;
    }
}
