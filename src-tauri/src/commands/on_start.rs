#[tauri::command]
pub fn on_start() {
    std::println!("Application started!");
}
