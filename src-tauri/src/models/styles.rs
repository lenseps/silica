use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::service::crud::Persistable;

pub type StyleId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    fn get_id(&self) -> &Self::Id {
        &self.id
    }
    fn load_meta_from(&mut self, old: &Self) {
        self.id = old.id;
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::styles::TaskStyle, service::crud::Persistable};

    #[test]
    fn test_persistent_load_meta_from_old_style() {
        let old_style: TaskStyle =
            TaskStyle::new("color".to_string(), "background_color".to_string());
        let mut new_style: TaskStyle =
            TaskStyle::new("color".to_string(), "background_color".to_string());

        new_style.load_meta_from(&old_style);

        assert_eq!(new_style.get_id(), old_style.get_id());
    }
}
