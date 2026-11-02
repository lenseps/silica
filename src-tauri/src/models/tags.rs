use uuid::Uuid;

use super::styles::TaskStyle;

pub type TagId = Uuid;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub style: Option<TaskStyle>,
}
