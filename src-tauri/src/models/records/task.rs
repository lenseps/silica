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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        models::records::{
            record::{Priority::DoFirst, Record, Status::Planned, TimeSpan},
            task::Task,
        },
        service::crud::Persistable,
    };

    fn generate_new_task() -> Task {
        let task_record: Record = Record {
            id: Uuid::new_v4(),
            title: "Title".to_string(),
            annotation: None,
            description: None,
            style: Some(Uuid::new_v4()),
            tags: Vec::new(),
            rrule: "Rrule".to_string(),
            created_at: Utc::now(),
        };
        let task_time: TimeSpan = TimeSpan {
            start_at: Utc::now(),
            end_at: Utc::now(),
        };
        let task: Task = Task {
            record: task_record,
            span: task_time,
            priority: DoFirst,
            status: Planned,
            deadline: None,
        };
        task
    }

    #[test]
    fn test_persistent_load_meta_from_old_task() {
        let old_task: Task = generate_new_task();
        let mut new_task: Task = generate_new_task();

        new_task.load_meta_from(&old_task);

        assert_eq!(new_task.get_id(), old_task.get_id());
        assert_eq!(new_task.record.created_at, old_task.record.created_at);
    }
}
