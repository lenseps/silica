use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

pub struct AppState {
    pub todos: Mutex<Vec<Todo>>,
    pub next_id: AtomicUsize,
}

#[tauri::command]
pub fn create_todo(state: tauri::State<'_, AppState>, title: String) -> Result<Todo, String> {
    let mut todos = state.todos.lock().map_err(|e| e.to_string())?;
    let new_id = state.next_id.fetch_add(1, Ordering::SeqCst);
    let new_todo = Todo {
        id: new_id,
        title,
        completed: false,
    };
    todos.push(new_todo.clone());
    Ok(new_todo)
}

#[tauri::command]
pub fn read_todos(state: tauri::State<'_, AppState>) -> Result<Vec<Todo>, String> {
    let todos = state.todos.lock().map_err(|e| e.to_string())?;
    Ok(todos.clone())
}

#[tauri::command]
pub fn update_todo(state: tauri::State<'_, AppState>, id: usize, completed: bool) -> Result<Todo, String> {
    let mut todos = state.todos.lock().map_err(|e| e.to_string())?;
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| "Task not found".to_string())?;
    todo.completed = completed;
    Ok(todo.clone())
}

#[tauri::command]
pub fn delete_todo(state: tauri::State<'_, AppState>, id: usize) -> Result<(), String> {
    let mut todos = state.todos.lock().map_err(|e| e.to_string())?;
    let initial_len = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < initial_len {
        Ok(())
    } else {
        Err("Task not found".into())
    }
}