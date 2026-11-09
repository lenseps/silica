use serde::{Deserialize, Serialize};

use crate::models::records::record::RecordId;
use crate::models::records::task::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum TaskRequest {
    Add { data: Task },
    GetAll,
    GetById { id: RecordId },
    Update { id: RecordId, data: Task },
    DeleteAll,
    DeleteById { id: RecordId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TaskResponse {
    Empty,
    Item { data: Task },
    List { data: Vec<Task> },
    Error { message: String },
}
