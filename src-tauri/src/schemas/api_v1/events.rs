use serde::{Deserialize, Serialize};

use crate::models::records::event::Event;
use crate::models::records::record::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum EventRequest {
    Add { data: Event },
    GetAll,
    GetById { id: RecordId },
    Update { id: RecordId, data: Event },
    DeleteAll,
    DeleteById { id: RecordId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EventResponse {
    Empty,
    Item { data: Event },
    List { data: Vec<Event> },
    Error { message: String },
}
