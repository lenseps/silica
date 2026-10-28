use super::tasks::TaskStyle;
use uuid::Uuid;

pub type TagId = Uuid;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub style: Option<TaskStyle>,
}