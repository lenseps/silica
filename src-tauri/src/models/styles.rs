use uuid::Uuid;

use crate::service::crud::Persistable;

pub type StyleId = Uuid;

#[derive(Debug, Clone)]
pub struct TaskStyle {
    pub id: StyleId,
    pub color: String,
    pub background_color: String,
}

impl TaskStyle {
    pub fn new(color: String, background_color: String) -> Self {
        TaskStyle {
            id: Uuid::new_v4(),
            color: color,
            background_color: background_color,
        }
    }
}

impl Persistable for TaskStyle {
    type Id = StyleId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn load_meta_from(&mut self, old: &Self) {
        self.id = old.id;
    }
}
