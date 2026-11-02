use uuid::Uuid;

pub type StyleId = Uuid;

#[derive(Debug, Clone)]
pub struct TaskStyle {
    pub id: StyleId,
    pub color: String,
    pub background_color: String,
}
