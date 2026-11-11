use tauri::State;

use crate::{
    AppState,
    commands::api_v1::{
        errors::CommandError,
        operations::{CrudCommandOperations, run_crud_command},
    },
    models::records::task::Task,
    schemas::api_v1::models::{ApiModelRequest, ApiModelResponse},
    service::crud::Crud,
};

pub type TaskRequest = ApiModelRequest<Task>;
pub type TaskResponse = ApiModelResponse<Task>;
struct TaskOperations<'a> {
    crud: &'a Crud<Task>,
}

impl<'a> TaskOperations<'a> {
    fn new(state: &'a State<'_, AppState>) -> Self {
        TaskOperations {
            crud: &state.tasks_crud,
        }
    }
}

impl<'a> CrudCommandOperations<Task> for TaskOperations<'a> {
    fn crud(&self) -> &crate::service::crud::Crud<Task> {
        &self.crud
    }
}

#[tauri::command]
pub async fn tasks<'a>(
    state: State<'a, AppState>,
    request: TaskRequest,
) -> Result<TaskResponse, CommandError> {
    let operations = TaskOperations::new(&state);
    run_crud_command(&operations, request).await
}
