use crate::models::tasks::task::{Task, TaskCategory, TaskData, TaskView};

#[derive(Debug, Clone)]
pub struct InstantTask {
    task: Task,
}

impl InstantTask {
    pub fn new(data: TaskData) -> Self {
        InstantTask { task: data.into() }
    }
}

impl TaskView for InstantTask {
    fn task(&self) -> &Task {
        &self.task
    }
    fn task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
    fn category(&self) -> TaskCategory {
        TaskCategory::Instant
    }
}
