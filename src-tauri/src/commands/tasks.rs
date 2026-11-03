use crate::{
    commands::errors::CommandError,
    schemas::api_v1::tasks::{TaskCreateRequest, TaskResponse},
};

#[tauri::command]
pub fn create_task(task: TaskCreateRequest) -> Result<TaskResponse, CommandError> {
    todo!()
}
