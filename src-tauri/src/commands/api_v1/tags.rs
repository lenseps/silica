use tauri::State;

use crate::{
    AppState,
    schemas::api_v1::tags::{TagRequest, TagResponse},
};

#[tauri::command]
pub async fn tags<'a>(state: State<'a, AppState>, request: TagRequest) -> Result<TagResponse, ()> {
    match request {
        TagRequest::Add { data } => Ok(tag_operations::add(&state.tags_crud, data).await),
        TagRequest::GetAll => Ok(tag_operations::get_all(&state.tags_crud).await),
        TagRequest::GetById { id } => Ok(tag_operations::get_by_id(&state.tags_crud, id).await),
        TagRequest::Update { id, data } => {
            Ok(tag_operations::update(&state.tags_crud, id, data).await)
        }
        TagRequest::DeleteAll => Ok(tag_operations::delete_all(&state.tags_crud).await),
        TagRequest::DeleteById { id } => {
            Ok(tag_operations::delete_by_id(&state.tags_crud, id).await)
        }
    }
}

mod tag_operations {
    use uuid::Uuid;

    use crate::{models::tags::Tag, schemas::api_v1::tags::TagResponse, service::crud::Crud};

    pub async fn add(crud: &Crud<Tag>, data: Tag) -> TagResponse {
        let add_result = crud.add(data.clone()).await;
        match add_result {
            Ok(_) => TagResponse::Item { data },
            Err(err) => TagResponse::Error {
                message: err.to_string(),
            },
        }
    }

    pub async fn get_all(crud: &Crud<Tag>) -> TagResponse {
        let get_result = crud.get_all().await;
        match get_result {
            Ok(data) => TagResponse::List { data: data },
            Err(err) => TagResponse::Error {
                message: err.to_string(),
            },
        }
    }

    pub async fn get_by_id(crud: &Crud<Tag>, id: Uuid) -> TagResponse {
        let get_result = crud.get_by_id(id).await;
        match get_result {
            Ok(opt_data) => match opt_data {
                Some(data) => TagResponse::Item { data: data },
                None => TagResponse::Empty,
            },
            Err(err) => TagResponse::Error {
                message: err.to_string(),
            },
        }
    }

    pub async fn update(crud: &Crud<Tag>, id: Uuid, mut new_data: Tag) -> TagResponse {
        new_data.id = id.clone();
        let update_result = crud.update(id, new_data).await;
        match update_result {
            Ok(_) => TagResponse::Empty,
            Err(err) => TagResponse::Error {
                message: err.to_string(),
            },
        }
    }

    pub async fn delete_all(crud: &Crud<Tag>) -> TagResponse {
        let delete_all_result = crud.delete_all().await;
        match delete_all_result {
            Ok(_) => TagResponse::Empty,
            Err(err) => TagResponse::Error {
                message: err.to_string(),
            },
        }
    }

    pub async fn delete_by_id(crud: &Crud<Tag>, id: Uuid) -> TagResponse {
        let delete_by_id_result = crud.get_by_id(id).await;
        match delete_by_id_result {
            Ok(opt_data) => match opt_data {
                Some(data) => TagResponse::Item { data: data },
                None => TagResponse::Empty,
            },
            Err(err) => TagResponse::Error {
                message: err.to_string(),
            },
        }
    }
}
