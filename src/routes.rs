use axum::Router;
use task_routes::task_routes;

mod task_routes;

use crate::model::app_state::AppState;

pub fn app_routes(state: AppState) -> Router {
    Router::new().nest("/api/task", task_routes(state.clone()))
}
