mod commands;
mod crud;

use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(crud::AppState {
            todos: Mutex::new(Vec::new()),
            next_id: AtomicUsize::new(1),
        })
        .setup(|_app| {
            commands::on_start::on_start();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crud::create_todo,
            crud::read_todos,
            crud::update_todo,
            crud::delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}