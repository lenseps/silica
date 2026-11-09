use serde::{Deserialize, Serialize};

use crate::models::styles::{StyleId, TaskStyle};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum StyleRequest {
    Add { data: TaskStyle },
    GetAll,
    GetById { id: StyleId },
    Update { id: StyleId, data: TaskStyle },
    DeleteAll,
    DeleteById { id: StyleId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StyleResponse {
    Empty,
    Item { data: TaskStyle },
    List { data: Vec<TaskStyle> },
    Error { message: String },
}
