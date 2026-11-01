use uuid::Uuid;

use crate::service::crud::Persistable;

pub type StyleId = Uuid;

#[derive(Debug, Clone)]
pub struct TaskStyle {
    pub id: StyleId,
    pub color: String,
    pub background_color: String,
}

impl Persistable for TaskStyle {
    type Id = StyleId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn load_meta_from_old(&mut self, old: &Self) {
        self.id = old.id;
    }
}
