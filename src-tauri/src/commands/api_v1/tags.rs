use tauri::State;

use crate::{
    AppState,
    commands::api_v1::operations::CrudCommandOperations,
    models::tags::{Tag, TagId},
    schemas::api_v1::models::{ApiModelRequest, ApiModelResponse},
};

pub type TagRequest = ApiModelRequest<TagId, Tag>;
pub type TagResponse = ApiModelResponse<Tag>;
struct TagOperations {}
impl CrudCommandOperations<Tag> for TagOperations {}

#[tauri::command]
pub async fn tags<'a>(state: State<'a, AppState>, request: TagRequest) -> Result<TagResponse, ()> {
    match request {
        TagRequest::Add { data } => Ok(TagOperations::add(&state.tags_crud, data).await),
        TagRequest::GetAll => Ok(TagOperations::get_all(&state.tags_crud).await),
        TagRequest::GetById { id } => Ok(TagOperations::get_by_id(&state.tags_crud, id).await),
        TagRequest::Update { id, data } => {
            Ok(TagOperations::update(&state.tags_crud, id, data).await)
        }
        TagRequest::DeleteAll => Ok(TagOperations::delete_all(&state.tags_crud).await),
        TagRequest::DeleteById { id } => {
            Ok(TagOperations::delete_by_id(&state.tags_crud, id).await)
        }
    }
}
