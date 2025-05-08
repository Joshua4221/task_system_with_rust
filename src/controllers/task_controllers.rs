use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    model::{app_state::AppState, task_model::CreateTask},
    Result,
};

#[axum::debug_handler]
pub async fn create_task(
    State(state): State<AppState>,
    Json(task): Json<CreateTask>,
) -> Result<impl IntoResponse> {
    let created_job = state.task_controller.creat_task(task).await?;

    Ok((StatusCode::CREATED, Json(created_job)))
}

#[axum::debug_handler]
pub async fn get_all_tasks(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let tasks = state.task_controller.get_all_task().await?;

    Ok((StatusCode::CREATED, Json(tasks)))
}

#[axum::debug_handler]
pub async fn get_single_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse> {
    let task = state.task_controller.get_single_task(id).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

#[axum::debug_handler]
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse> {
    let task = state.task_controller.delete_task(id).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

#[axum::debug_handler]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(task): Json<CreateTask>,
) -> Result<impl IntoResponse> {
    let task = state.task_controller.update_task(id, task).await?;

    Ok((StatusCode::CREATED, Json(task)))
}
