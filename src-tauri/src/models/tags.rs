use uuid::Uuid;

use super::styles::TaskStyle;
use crate::service::crud::Persistable;

pub type TagId = Uuid;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub style: Option<TaskStyle>,
}

impl Persistable for Tag {
    type Id = TagId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn load_meta_from_old(&mut self, old: &Self) {
        self.id = old.id
    }
}
