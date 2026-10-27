#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::on_start::on_start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
