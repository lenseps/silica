use tauri::State;

use crate::{
    AppState,
    commands::api_v1::{
        errors::CommandError,
        operations::{CrudCommandOperations, run_crud_command},
    },
    models::tags::Tag,
    schemas::api_v1::models::{ApiModelRequest, ApiModelResponse},
    service::crud::Crud,
};

pub type TagRequest = ApiModelRequest<Tag>;
pub type TagResponse = ApiModelResponse<Tag>;
struct TagOperations<'a> {
    crud: &'a Crud<Tag>,
}

impl<'a> TagOperations<'a> {
    fn new(state: &'a State<'_, AppState>) -> Self {
        TagOperations {
            crud: &state.tags_crud,
        }
    }
}

impl<'a> CrudCommandOperations<Tag> for TagOperations<'a> {
    fn crud(&self) -> &crate::service::crud::Crud<Tag> {
        &self.crud
    }
}

#[tauri::command]
pub async fn tags<'a>(
    state: State<'a, AppState>,
    request: TagRequest,
) -> Result<TagResponse, CommandError> {
    let operations = TagOperations::new(&state);
    run_crud_command(&operations, request).await
}
