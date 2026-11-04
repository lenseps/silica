use serde::{Deserialize, Serialize};

use crate::{
    impl_persistable, impl_shedulable,
    models::records::record::{HasRecord, HasTimeSpan, Record, RecordId, TimeSpan},
    service::crud::Persistable,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub record: Record,
    #[serde(flatten)]
    pub span: TimeSpan,
}

impl_persistable!(Event);
impl_shedulable!(Event);
