use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::styles::StyleId, service::crud::Persistable};

pub type TagId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub style: Option<StyleId>,
}

impl Tag {
    pub fn new(name: String, style: Option<StyleId>) -> Self {
        Tag {
            id: TagId::new_v4(),
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
        models::{styles::StyleId, tags::Tag},
        service::crud::Persistable,
    };

    #[test]
    fn test_persistent_load_meta_from_old_tag() {
        let old_tag: Tag = Tag::new("Old".to_string(), Some(StyleId::new_v4()));
        let mut new_tag: Tag = Tag::new("New".to_string(), None);

        new_tag.load_meta_from(&old_tag);

        assert_eq!(new_tag.get_id(), old_tag.get_id());
    }
}
