use tauri::generate_handler;

use crate::{models::tags::Tag, service::crud::Crud};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod models;
mod schemas;
mod service;

const TAGS_CRUD_PATH: &str = "./tags.json";

struct AppState {
    tags_crud: Crud<Tag>,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            tags_crud: Crud::new(TAGS_CRUD_PATH),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![crate::commands::api_v1::tags::tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
