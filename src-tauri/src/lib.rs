#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod models;
mod schemas;
mod service;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::tasks::create_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
