use super::task_model::TaskController;

#[derive(Clone)]
pub struct AppState {
    pub task_controller: TaskController,
}
