use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

pub type Timestamp = DateTime<Utc>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    OneTime,
    Interval,
    Instant,
}
pub struct TaskData {
    pub id: u64,
    pub title: String,
    /// Annotation is a brief description below the title
    /// that is displayed without opening the full details of a task.
    pub annotation: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Task {
    id: u64,
    uuid: Uuid,
    title: String,
    annotation: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    created: Timestamp,
    ended: Option<Timestamp>,
}

impl From<TaskData> for Task {
    fn from(d: TaskData) -> Self {
        Task {
            id: d.id,
            uuid: Uuid::new_v4(),
            title: d.title,
            annotation: d.annotation,
            description: d.description,
            tags: d.tags,
            created: Utc::now(),
            ended: None,
        }
    }
}

pub trait TaskView {
    fn task(&self) -> &Task;
    fn task_mut(&mut self) -> &mut Task;
    fn status(&self) -> TaskStatus;

    fn id(&self) -> u64 {
        self.task().id
    }
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
    fn tags(&self) -> &[String] {
        &self.task().tags
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
    interval: Duration,
}

impl TaskView for OneTimeTask {
    fn task(&self) -> &Task {
        &self.task
    }
    fn task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
    fn status(&self) -> TaskStatus {
        TaskStatus::OneTime
    }
}

impl TaskView for InstantTask {
    fn task(&self) -> &Task {
        &self.task
    }
    fn task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
    fn status(&self) -> TaskStatus {
        TaskStatus::Instant
    }
}

impl TaskView for IntervalTask {
    fn task(&self) -> &Task {
        &self.task
    }
    fn task_mut(&mut self) -> &mut Task {
        &mut self.task
    }
    fn status(&self) -> TaskStatus {
        TaskStatus::Interval
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
    pub fn new(data: TaskData, interval: Duration) -> Self {
        IntervalTask {
            task: data.into(),
            interval,
        }
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }
}
