use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{styles::StyleId, tags::TagId};

pub type RecordId = Uuid;
pub type Timestamp = DateTime<Utc>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    DoFirst,
    Schedule,
    Delegate,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Planned,
    Canceled,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: RecordId,
    pub title: String,
    // Annotation is a shorter description that displays below the name.
    pub annotation: Option<String>,
    pub description: Option<String>,
    pub style: Option<StyleId>,
    pub tags: Vec<TagId>,
    // Rrule is a recurrence rule.
    pub rrule: String,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeSpan {
    pub start_at: Timestamp,
    pub end_at: Timestamp,
}

pub trait HasRecord {
    fn record(&self) -> &Record;
    fn record_mut(&mut self) -> &mut Record;
}

pub trait HasTimeSpan {
    fn span(&self) -> &TimeSpan;
    fn span_mut(&mut self) -> &mut TimeSpan;
}

pub trait Schedulable: HasRecord + HasTimeSpan {}
impl<T: HasRecord + HasTimeSpan> Schedulable for T {}

#[macro_export]
macro_rules! impl_shedulable {
    ($($t:ty),* $(,)?) => {$(
        impl HasRecord for $t {
            fn record(&self) -> &Record { &self.record }
            fn record_mut(&mut self) -> &mut Record { &mut self.record }
        }
        impl HasTimeSpan for $t {
            fn span(&self) -> &TimeSpan { &self.span }
            fn span_mut(&mut self) -> &mut TimeSpan { &mut self.span }
        }
    )*};
}

#[macro_export]
macro_rules! impl_persistable {
    ($($t:ty),+) => {
        $(
            impl Persistable for $t {
                type Id = RecordId;

                fn get_id(&self) -> &Self::Id {
                    &self.record.id
                }

                fn load_meta_from(&mut self, old: &Self) {
                    self.record.id = old.record.id;
                    self.record.created_at = old.record.created_at;
                }
            }
        )*
    };
}
