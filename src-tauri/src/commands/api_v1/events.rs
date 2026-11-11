use tauri::State;

use crate::{
    AppState,
    commands::api_v1::{
        errors::CommandError,
        operations::{CrudCommandOperations, run_crud_command},
    },
    models::records::event::Event,
    schemas::api_v1::models::{ApiModelRequest, ApiModelResponse},
    service::crud::Crud,
};

pub type EventRequest = ApiModelRequest<Event>;
pub type EventResponse = ApiModelResponse<Event>;
struct EventOperations<'a> {
    crud: &'a Crud<Event>,
}

impl<'a> EventOperations<'a> {
    fn new(state: &'a State<'_, AppState>) -> Self {
        EventOperations {
            crud: &state.events_crud,
        }
    }
}

impl<'a> CrudCommandOperations<Event> for EventOperations<'a> {
    fn crud(&self) -> &crate::service::crud::Crud<Event> {
        &self.crud
    }
}

#[tauri::command]
pub async fn events<'a>(
    state: State<'a, AppState>,
    request: EventRequest,
) -> Result<EventResponse, CommandError> {
    let operations = EventOperations::new(&state);
    run_crud_command(&operations, request).await
}
