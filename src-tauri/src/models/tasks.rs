use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::tags::TagId;

pub type Timestamp = DateTime<Utc>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskCategory {
    OneTime,
    Interval,
    Instant,
}

#[derive(Debug, Clone)]
pub struct TaskStyle {
    pub color: String,
    pub background_color: String,
}

#[derive(Debug, Clone)]
pub struct TaskData {
    pub title: String,
    /// Annotation is a brief description below the title
    /// that is displayed without opening the full details of a task.
    pub annotation: Option<String>,
    pub description: Option<String>,
    pub style: Option<TaskStyle>,
    pub tags: Vec<TagId>,
}

#[derive(Debug, Clone)]
pub struct Task {
    uuid: Uuid,
    title: String,
    annotation: Option<String>,
    description: Option<String>,
    tags: Vec<TagId>,
    style: Option<TaskStyle>,
    created: Timestamp,
    ended: Option<Timestamp>,
}

impl From<TaskData> for Task {
    fn from(d: TaskData) -> Self {
        Task {
            uuid: Uuid::new_v4(),
            title: d.title,
            annotation: d.annotation,
            description: d.description,
            tags: d.tags,
            style: d.style,
            created: Utc::now(),
            ended: None,
        }
    }
}

pub trait TaskView {
    fn task(&self) -> &Task;
    fn task_mut(&mut self) -> &mut Task;
    fn category(&self) -> TaskCategory;
    fn uuid(&self) -> Uuid {
        self.task().uuid
    }
    fn created(&self) -> Timestamp {
        self.task().created
    }
    fn title(&self) -> &str {
        &self.task().title
    }
    fn annotation(&self) -> Option<&str> {
        self.task().annotation.as_deref()
    }
    fn description(&self) -> Option<&str> {
        self.task().description.as_deref()
    }
    fn tags(&self) -> &[TagId] {
        &self.task().tags
    }
    fn style(&self) -> Option<&TaskStyle> {
        self.task().style.as_ref()
    }
    fn ended(&self) -> Option<Timestamp> {
        self.task().ended
    }
    fn end(&mut self) {
        self.task_mut().ended = Some(Utc::now());
    }
}

#[derive(Debug, Clone)]
pub struct OneTimeTask {
    task: Task,
}

#[derive(Debug, Clone)]
pub struct InstantTask {
    task: Task,
}

#[derive(Debug, Clone)]
pub struct IntervalTask {
    task: Task,
    interval_seconds: u64,
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

impl TaskView for IntervalTask {
    fn task(&self) -> &Task {
        &self.task
    }
    fn task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
    fn category(&self) -> TaskCategory {
        TaskCategory::Interval
    }
}

impl OneTimeTask {
    pub fn new(data: TaskData) -> Self {
        OneTimeTask { task: data.into() }
    }
}

impl InstantTask {
    pub fn new(data: TaskData) -> Self {
        InstantTask { task: data.into() }
    }
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
