use crate::{
    impl_task_view_for_tasks,
    models::tasks::task::{Task, TaskCategory, TaskData, TaskId, TaskView},
    service::crud::Persistable,
};

#[derive(Debug, Clone)]
pub struct OneTimeTask {
    task: Task,
}

impl OneTimeTask {
    pub fn new(data: TaskData) -> Self {
        OneTimeTask { task: data.into() }
    }
}

impl_task_view_for_tasks!(OneTimeTask: TaskCategory::OneTime);
