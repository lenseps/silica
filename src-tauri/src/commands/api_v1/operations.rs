use serde::{Serialize, de::DeserializeOwned};

use crate::commands::api_v1::errors::CommandError;
use crate::schemas::api_v1::models::{ApiModelRequest, ApiModelResponse};
use crate::service::crud::{Crud, Persistable};

pub fn match_operation_result<T>(
    result: ApiModelResponse<T>,
) -> Result<ApiModelResponse<T>, CommandError> {
    match &result {
        ApiModelResponse::Error { message } => Err(CommandError::UnknownError(message.clone())),
        _ => Ok(result),
    }
}
pub trait CrudCommandOperations<T>
where
    T: Clone + Serialize + DeserializeOwned + Persistable,
    <T as Persistable>::Id: PartialEq,
{
    fn crud(&self) -> &Crud<T>;

    async fn add(&self, data: T) -> ApiModelResponse<T> {
        let add_result = self.crud().add(data.clone()).await;
        match add_result {
            Ok(_) => ApiModelResponse::Item { data },
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn get_all(&self) -> ApiModelResponse<T> {
        let get_result = self.crud().get_all().await;
        match get_result {
            Ok(data) => ApiModelResponse::List { data },
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn get_by_id(&self, id: T::Id) -> ApiModelResponse<T> {
        let get_result = self.crud().get_by_id(id).await;
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

    async fn update(&self, id: T::Id, new_data: T) -> ApiModelResponse<T> {
        let update_result = self.crud().update(id, new_data).await;
        match update_result {
            Ok(_) => ApiModelResponse::Empty,
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn delete_all(&self) -> ApiModelResponse<T> {
        let delete_all_result = self.crud().delete_all().await;
        match delete_all_result {
            Ok(_) => ApiModelResponse::Empty,
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }

    async fn delete_by_id(&self, id: T::Id) -> ApiModelResponse<T> {
        let delete_by_id_result = self.crud().delete_by_id(id).await;
        match delete_by_id_result {
            Ok(_) => ApiModelResponse::Empty,
            Err(err) => ApiModelResponse::Error {
                message: err.to_string(),
            },
        }
    }
}

pub async fn run_crud_command<T>(
    operations: &impl CrudCommandOperations<T>,
    request: ApiModelRequest<T>,
) -> Result<ApiModelResponse<T>, CommandError>
where
    T: Clone + Serialize + DeserializeOwned + Persistable,
    <T as Persistable>::Id: PartialEq,
{
    use ApiModelRequest::*;
    match request {
        Add { data } => {
            let result = operations.add(data).await;
            match_operation_result(result)
        }
        GetAll => {
            let result = operations.get_all().await;
            match_operation_result(result)
        }
        GetById { id } => {
            let result = operations.get_by_id(id).await;
            match_operation_result(result)
        }
        Update { id, data } => {
            let result = operations.update(id, data).await;
            match_operation_result(result)
        }
        DeleteAll => {
            let result = operations.delete_all().await;
            match_operation_result(result)
        }
        DeleteById { id } => {
            let result = operations.delete_by_id(id).await;
            match_operation_result(result)
        }
    }
}
