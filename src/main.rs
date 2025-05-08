pub use error::{Error, Result};
use model::{app_state::AppState, task_model::TaskController};
use routes::app_routes;
use tokio::net::TcpListener;

mod controllers;
mod error;
mod model;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let task_controller = TaskController::new().await?;

    let state = AppState { task_controller };

    // Set up routes and inject shared state
    let app = app_routes(state);

    // Bind the TCP listener
    let listener = TcpListener::bind("127.0.0.1:8010").await.unwrap();

    // Start the Axum server
    println!("🚀 Server running at http://127.0.0.1:8010");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
