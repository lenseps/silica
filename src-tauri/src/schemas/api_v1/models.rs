use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ApiModelRequest<IdT, T> {
    Add { data: T },
    GetAll,
    GetById { id: IdT },
    Update { id: IdT, data: T },
    DeleteAll,
    DeleteById { id: IdT },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ApiModelResponse<T> {
    Empty,
    Item { data: T },
    List { data: Vec<T> },
    Error { message: String },
}
