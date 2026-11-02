use crate::models::tasks::task::{Task, TaskCategory, TaskData, TaskView};

#[derive(Debug, Clone)]
pub struct OneTimeTask {
    task: Task,
}

impl OneTimeTask {
    pub fn new(data: TaskData) -> Self {
        OneTimeTask { task: data.into() }
    }
}

impl TaskView for OneTimeTask {
    fn task(&self) -> &Task {
        &self.task
    }
    fn task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
    fn category(&self) -> TaskCategory {
        TaskCategory::OneTime
    }
}
