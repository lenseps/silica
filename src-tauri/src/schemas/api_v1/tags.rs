use serde::{Deserialize, Serialize};

use crate::models::tags::{Tag, TagId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum TagRequest {
    Add { data: Tag },
    GetAll,
    GetById { id: TagId },
    Update { id: TagId, data: Tag },
    DeleteAll,
    DeleteById { id: TagId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TagResponse {
    Empty,
    Item { data: Tag },
    List { data: Vec<Tag> },
    Error { message: String },
}
