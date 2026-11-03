use crate::{
    commands::errors::CommandError,
    models::tasks::task::{TaskData, TaskId},
    schemas::api_v1::tasks::{TaskCreateRequest, TaskResponse},
};

#[tauri::command]
pub fn create_task(task: TaskCreateRequest) -> Result<TaskResponse, CommandError> {
    todo!()
}

#[tauri::command]
pub fn get_task_by_id(id: TaskId) -> Result<TaskResponse, CommandError> {
    todo!()
}

#[tauri::command]
pub fn get_tasks() -> Result<Vec<TaskResponse>, CommandError> {
    todo!()
}

#[tauri::command]
pub fn update_task(id: TaskId, new_data: TaskData) -> Result<TaskResponse, CommandError> {
    todo!()
}
