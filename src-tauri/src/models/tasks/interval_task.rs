use crate::{
    impl_task_view_for_tasks,
    models::tasks::task::{Task, TaskCategory, TaskData, TaskView},
};

#[derive(Debug, Clone)]
pub struct IntervalTask {
    task: Task,
    interval_seconds: u64,
}

impl IntervalTask {
    pub fn new(data: TaskData, interval_seconds: u64) -> Self {
        IntervalTask {
            task: data.into(),
            interval_seconds,
        }
    }
    pub fn interval_seconds(&self) -> u64 {
        self.interval_seconds
    }
}

impl_task_view_for_tasks!(IntervalTask: TaskCategory::Interval);
