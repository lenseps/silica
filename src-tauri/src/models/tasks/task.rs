use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::styles::StyleId;
use crate::models::tags::TagId;

pub type TaskId = Uuid;

pub type Timestamp = DateTime<Utc>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskCategory {
    OneTime,
    Interval,
    Instant,
}

#[derive(Debug, Clone)]
pub struct TaskData {
    pub title: String,
    /// Annotation is a brief description below the title
    /// that is displayed without opening the full details of a task.
    pub annotation: Option<String>,
    pub description: Option<String>,
    pub style: Option<StyleId>,
    pub tags: Vec<TagId>,
}

#[derive(Debug, Clone)]
pub struct Task {
    id: TaskId,
    title: String,
    annotation: Option<String>,
    description: Option<String>,
    tags: Vec<TagId>,
    style: Option<StyleId>,
    created: Timestamp,
    ended: Option<Timestamp>,
}

impl From<TaskData> for Task {
    fn from(d: TaskData) -> Self {
        Task {
            id: TaskId::new_v4(),
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
    fn id(&self) -> TaskId {
        self.task().id
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
    fn style(&self) -> Option<&StyleId> {
        self.task().style.as_ref()
    }
    fn ended(&self) -> Option<Timestamp> {
        self.task().ended
    }
    fn end(&mut self) {
        self.task_mut().ended = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::models::tasks::{
        InstantTask, IntervalTask, OneTimeTask, TaskCategory, TaskData, task::TaskView,
    };

    const TITLE: &str = "Title";
    const ANNOTATION: &str = "Annotation";
    const DESRIPTION: &str = "Description";

    fn generate_task_data() -> TaskData {
        TaskData {
            title: TITLE.to_string(),
            annotation: Some(ANNOTATION.to_string()),
            description: Some(DESRIPTION.to_string()),
            style: None,
            tags: Vec::new(),
        }
    }

    #[test]
    fn test_create_new_one_time_task() {
        let data = generate_task_data();

        let task = OneTimeTask::new(data);

        assert_eq!(task.title(), TITLE);
        assert_eq!(task.annotation(), Some(ANNOTATION));
        assert_eq!(task.description(), Some(DESRIPTION));
        assert_matches!(task.style(), None);
        assert_eq!(task.tags().len(), 0);
        assert_matches!(task.category(), TaskCategory::OneTime);
    }

    #[test]
    fn test_create_new_instant_task() {
        let data = generate_task_data();

        let task = InstantTask::new(data);

        assert_eq!(task.title(), TITLE);
        assert_eq!(task.annotation(), Some(ANNOTATION));
        assert_eq!(task.description(), Some(DESRIPTION));
        assert_matches!(task.style(), None);
        assert_eq!(task.tags().len(), 0);
        assert_matches!(task.category(), TaskCategory::Instant);
    }

    #[test]
    fn test_create_new_interval_task() {
        let data = generate_task_data();
        let interval_seconds = 100u64;

        let task = IntervalTask::new(data, interval_seconds);

        assert_eq!(task.title(), TITLE);
        assert_eq!(task.annotation(), Some(ANNOTATION));
        assert_eq!(task.description(), Some(DESRIPTION));
        assert_matches!(task.style(), None);
        assert_eq!(task.tags().len(), 0);
        assert_eq!(task.interval_seconds(), interval_seconds);
        assert_matches!(task.category(), TaskCategory::Interval);
    }
}
