На данный момент в проекте присутствуют основные **четыре** публичные модели.

1. Style
2. Tag
3. Task
4. Event

Ниже приведены `JSON`-структуры, описывающие все их поля.

```rust
pub struct Style {
    pub id: Uuid,
    pub color: String,
    pub background_color: String,
}
```

```rust
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub style: Option<Uuid>, // Style id, nullable field.
}
```

```rust
pub struct Task {
    pub id: Uuid,
    pub title: String,
    // Annotation is a shorter description that displays below the name.
    pub annotation: Option<String>, // nullable field.
    pub description: Option<String>, // nullable field.
    pub style: Option<Uuid>, // Style id, nullable field.
    pub tags: Vec<Uuid>, // Tag id, array field.
    // Rrule is a recurrence rule.
    pub rrule: String,
    pub created_at: Timestamp,

    pub start_at: Timestamp,
    pub end_at: Timestamp,

    pub priority: Priority, // Enum, can be:  DoFirst, Schedule, Delegate, Delete.
    pub status: Status, // Enum, can be: Planned, Canceled, InProgress, Finished.
    pub deadline: Option<Timestamp>, // nullable field.
}
```

```rust
pub struct Event {
    pub id: Uuid,
    pub title: String,
    // Annotation is a shorter description that displays below the name.
    pub annotation: Option<String>, // nullable field.
    pub description: Option<String>, // nullable field.
    pub style: Option<Uuid>, // Style id, nullable field.
    pub tags: Vec<Uuid>, // Tag id, array field.
    // Rrule is a recurrence rule.
    pub rrule: String,
    pub created_at: Timestamp,

    pub start_at: Timestamp,
    pub end_at: Timestamp,
}
```
