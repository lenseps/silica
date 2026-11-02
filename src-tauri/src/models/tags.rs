use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::styles::TaskStyle;
use crate::service::crud::Persistable;

pub type TagId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub style: Option<TaskStyle>,
}

impl Tag {
    pub fn new(name: String, style: Option<TaskStyle>) -> Self {
        Tag {
            id: Uuid::new_v4(),
            name: name,
            style: style,
        }
    }
}

impl Persistable for Tag {
    type Id = TagId;

    fn get_id(&self) -> &Self::Id {
        &self.id
    }
    fn load_meta_from(&mut self, old: &Self) {
        self.id = old.id
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{styles::TaskStyle, tags::Tag},
        service::crud::Persistable,
    };

    #[test]
    fn test_load_meta_from_old_works_correcly() {
        let style: TaskStyle = TaskStyle::new("color".to_string(), "background_color".to_string());
        let old_tag: Tag = Tag::new("Old".to_string(), Some(style.clone()));
        let mut new_tag: Tag = Tag::new("New".to_string(), Some(style));

        new_tag.load_meta_from(&old_tag);

        assert_eq!(new_tag.get_id(), old_tag.get_id());
    }
}
