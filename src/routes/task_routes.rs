use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::task_controllers::{
        create_task, delete_task, get_all_tasks, get_single_task, update_task,
    },
    model::app_state::AppState,
};

pub fn task_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_task).get(get_all_tasks))
        .route(
            "/{id}",
            get(get_single_task).delete(delete_task).patch(update_task),
        )
        .with_state(state)
}
