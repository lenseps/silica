use serde::{Deserialize, Serialize};

use crate::{
    impl_persistable, impl_shedulable,
    models::records::record::{
        HasRecord, HasTimeSpan, Priority, Record, RecordId, Status, TimeSpan, Timestamp,
    },
    service::crud::Persistable,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(flatten)]
    pub record: Record,
    #[serde(flatten)]
    pub span: TimeSpan,
    pub priority: Priority,
    pub status: Status,
    pub deadline: Option<Timestamp>,
}

impl_persistable!(Task);
impl_shedulable!(Task);
