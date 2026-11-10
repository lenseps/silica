use serde::{Deserialize, Serialize};

use crate::service::crud::Persistable;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ApiModelRequest<T: Persistable> {
    Add { data: T },
    GetAll,
    GetById { id: T::Id },
    Update { id: T::Id, data: T },
    DeleteAll,
    DeleteById { id: T::Id },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ApiModelResponse<T> {
    Empty,
    Item { data: T },
    List { data: Vec<T> },
    Error { message: String },
}
