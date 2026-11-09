use crate::schemas::api_v1::tags::{TagRequest, TagResponse};

#[tauri::command]
async fn tags(request: TagRequest) -> TagResponse {
    todo!()
}
