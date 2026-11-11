use tauri::State;

use crate::{
    AppState,
    commands::api_v1::{
        errors::CommandError,
        operations::{CrudCommandOperations, run_crud_command},
    },
    models::styles::TaskStyle,
    schemas::api_v1::models::{ApiModelRequest, ApiModelResponse},
    service::crud::Crud,
};

pub type TaskStyleRequest = ApiModelRequest<TaskStyle>;
pub type TaskStyleResponse = ApiModelResponse<TaskStyle>;
struct TaskStyleOperations<'a> {
    crud: &'a Crud<TaskStyle>,
}

impl<'a> TaskStyleOperations<'a> {
    fn new(state: &'a State<'_, AppState>) -> Self {
        TaskStyleOperations {
            crud: &state.styles_crud,
        }
    }
}

impl<'a> CrudCommandOperations<TaskStyle> for TaskStyleOperations<'a> {
    fn crud(&self) -> &crate::service::crud::Crud<TaskStyle> {
        &self.crud
    }
}

#[tauri::command]
pub async fn styles<'a>(
    state: State<'a, AppState>,
    request: TaskStyleRequest,
) -> Result<TaskStyleResponse, CommandError> {
    let operations = TaskStyleOperations::new(&state);
    run_crud_command(&operations, request).await
}
