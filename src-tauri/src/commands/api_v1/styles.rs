use tauri::State;

use crate::{
    AppState,
    commands::api_v1::{
        errors::CommandError,
        operations::{CrudCommandOperations, run_crud_command},
    },
    models::styles::Style,
    schemas::api_v1::models::{ApiModelRequest, ApiModelResponse},
    service::crud::Crud,
};

pub type StyleRequest = ApiModelRequest<Style>;
pub type StyleResponse = ApiModelResponse<Style>;
struct StyleOperations<'a> {
    crud: &'a Crud<Style>,
}

impl<'a> StyleOperations<'a> {
    fn new(state: &'a State<'_, AppState>) -> Self {
        StyleOperations {
            crud: &state.styles_crud,
        }
    }
}

impl<'a> CrudCommandOperations<Style> for StyleOperations<'a> {
    fn crud(&self) -> &crate::service::crud::Crud<Style> {
        &self.crud
    }
}

#[tauri::command]
pub async fn styles<'a>(
    state: State<'a, AppState>,
    request: StyleRequest,
) -> Result<StyleResponse, CommandError> {
    let operations = StyleOperations::new(&state);
    run_crud_command(&operations, request).await
}
