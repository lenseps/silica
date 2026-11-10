use serde::{Serialize, de::DeserializeOwned};

use crate::schemas::api_v1::models::ApiModelResponse;
use crate::service::crud::{Crud, Persistable};

pub trait CrudCommandOperations<T>
where
    T: Clone + Serialize + DeserializeOwned + Persistable,
    <T as Persistable>::Id: PartialEq,
{
    async fn add(crud: &Crud<T>, data: T) -> ApiModelResponse<T> {
        let add_result = crud.add(data.clone()).await;
        match add_result {
            Ok(_) => ApiModelResponse::Item { data },
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn get_all(crud: &Crud<T>) -> ApiModelResponse<T> {
        let get_result = crud.get_all().await;
        match get_result {
            Ok(data) => ApiModelResponse::List { data },
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn get_by_id(crud: &Crud<T>, id: T::Id) -> ApiModelResponse<T> {
        let get_result = crud.get_by_id(id).await;
        match get_result {
            Ok(opt_data) => match opt_data {
                Some(data) => ApiModelResponse::Item { data },
                None => ApiModelResponse::Empty,
            },
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn update(crud: &Crud<T>, id: T::Id, new_data: T) -> ApiModelResponse<T> {
        let update_result = crud.update(id, new_data).await;
        match update_result {
            Ok(_) => ApiModelResponse::Empty,
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn delete_all(crud: &Crud<T>) -> ApiModelResponse<T> {
        let delete_all_result = crud.delete_all().await;
        match delete_all_result {
            Ok(_) => ApiModelResponse::Empty,
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn delete_by_id(crud: &Crud<T>, id: T::Id) -> ApiModelResponse<T> {
        let delete_by_id_result = crud.delete_by_id(id).await;
        match delete_by_id_result {
            Ok(_) => ApiModelResponse::Empty,
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }
}
