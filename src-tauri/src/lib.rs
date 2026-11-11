use tauri::generate_handler;

use crate::{
    models::{
        records::{event::Event, task::Task},
        styles::TaskStyle,
        tags::Tag,
    },
    service::crud::Crud,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod models;
mod schemas;
mod service;

const TASKS_CRUD_PATH: &str = "./tags.json";
const EVENTS_CRUD_PATH: &str = "./tags.json";
const TAGS_CRUD_PATH: &str = "./tags.json";
const STYLES_CRUD_PATH: &str = "./tags.json";

struct AppState {
    tasks_crud: Crud<Task>,
    events_crud: Crud<Event>,
    tags_crud: Crud<Tag>,
    styles_crud: Crud<TaskStyle>,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            tasks_crud: Crud::new(TASKS_CRUD_PATH),
            events_crud: Crud::new(EVENTS_CRUD_PATH),
            tags_crud: Crud::new(TAGS_CRUD_PATH),
            styles_crud: Crud::new(STYLES_CRUD_PATH),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![
            crate::commands::api_v1::tags::tags,
            crate::commands::api_v1::events::events,
            crate::commands::api_v1::tasks::tasks,
            crate::commands::api_v1::styles::styles,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
