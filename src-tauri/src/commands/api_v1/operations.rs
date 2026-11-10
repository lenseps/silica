use serde::{Serialize, de::DeserializeOwned};

use crate::commands::api_v1::errors::CommandError;
use crate::schemas::api_v1::models::{ApiModelRequest, ApiModelResponse};
use crate::service::crud::{Crud, CrudError, Persistable};

pub fn match_operation_result<T>(
    result: Result<ApiModelResponse<T>, CrudError>,
) -> Result<ApiModelResponse<T>, CommandError> {
    match result {
        Ok(model) => Ok(model),
        Err(err) => Err(CommandError::CrudError(err.to_string())),
    }
}
pub trait CrudCommandOperations<T>
where
    T: Clone + Serialize + DeserializeOwned + Persistable,
    <T as Persistable>::Id: PartialEq,
{
    fn crud(&self) -> &Crud<T>;

    async fn add(&self, data: T) -> Result<ApiModelResponse<T>, CrudError> {
        self.crud().add(data.clone()).await?;
        Ok(ApiModelResponse::Item { data })
    }

    async fn get_all(&self) -> Result<ApiModelResponse<T>, CrudError> {
        let get_result = self.crud().get_all().await?;
        Ok(ApiModelResponse::List { data: get_result })
    }

    async fn get_by_id(&self, id: T::Id) -> Result<ApiModelResponse<T>, CrudError> {
        let get_result = self.crud().get_by_id(id).await?;
        match get_result {
            Some(data) => Ok(ApiModelResponse::Item { data }),
            None => Ok(ApiModelResponse::Empty),
        }
    }

    async fn update(&self, id: T::Id, new_data: T) -> Result<ApiModelResponse<T>, CrudError> {
        self.crud().update(id, new_data).await?;
        Ok(ApiModelResponse::Empty)
    }

    async fn delete_all(&self) -> Result<ApiModelResponse<T>, CrudError> {
        self.crud().delete_all().await?;
        Ok(ApiModelResponse::Empty)
    }

    async fn delete_by_id(&self, id: T::Id) -> Result<ApiModelResponse<T>, CrudError> {
        self.crud().delete_by_id(id).await?;
        Ok(ApiModelResponse::Empty)
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
