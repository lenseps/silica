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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        models::records::{
            event::Event,
            record::{Record, TimeSpan},
        },
        service::crud::Persistable,
    };

    fn generate_new_event() -> Event {
        let event_record: Record = Record {
            id: Uuid::new_v4(),
            title: "Title".to_string(),
            annotation: None,
            description: None,
            style: Some(Uuid::new_v4()),
            tags: Vec::new(),
            rrule: "Rrule".to_string(),
            created_at: Utc::now(),
        };
        let event_time: TimeSpan = TimeSpan {
            start_at: Utc::now(),
            end_at: Utc::now(),
        };
        let event: Event = Event {
            record: event_record,
            span: event_time,
        };
        event
    }

    #[test]
    fn test_persistent_load_meta_from_old_event() {
        let old_event: Event = generate_new_event();
        let mut new_event: Event = generate_new_event();

        new_event.load_meta_from(&old_event);

        assert_eq!(new_event.get_id(), old_event.get_id());
        assert_eq!(new_event.record.created_at, old_event.record.created_at);
    }
}
