use chrono::{DateTime, Utc};
use serde::{Deserializa, Serializa};

//TODO Structure data task and event. To quickly search by event time.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct timeLineEvent{
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub rrule: Option<String>, // for repetitive tasks
    pub color: Option<String>,
}