use crate::{
    impl_task_view_for_tasks,
    models::tasks::task::{Task, TaskCategory, TaskData, TaskView},
};

#[derive(Debug, Clone)]
pub struct InstantTask {
    task: Task,
}

impl InstantTask {
    pub fn new(data: TaskData) -> Self {
        InstantTask { task: data.into() }
    }
}

impl_task_view_for_tasks!(InstantTask: TaskCategory::Instant);
