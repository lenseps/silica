use serde::{Deserialize, Serialize};

use crate::models::{
    tags::TagId,
    tasks::task::{TaskCategory, TaskId, Timestamp},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreateRequest {
    pub category: TaskCategory,
    pub title: String,
    pub annotation: Option<String>,
    pub description: Option<String>,
    pub style: Option<TaskId>,
    pub tags: Vec<TagId>,
    pub interval_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: TaskId,
    pub category: TaskCategory,
    pub title: String,
    pub annotation: Option<String>,
    pub description: Option<String>,
    pub style: Option<TaskId>,
    pub tags: Vec<TagId>,
    pub interval_seconds: Option<u64>,
    pub created: Timestamp,
    pub ended: Option<Timestamp>,
}
