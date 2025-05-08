# Rust Task Management API

A simple yet extensible **Task Management REST API** built with **Rust**, using **Axum** as the web framework and `tokio` for asynchronous execution. This project demonstrates clean architecture principles, modular design, and safe concurrency using shared state with `Arc<Mutex<_>>`.

---

## 📁 Project Structure

```
.
├── Cargo.toml
├── src/
│   ├── main.rs                # Application entry point
│   ├── error.rs               # Custom error types and handler
│   ├── model/
│   │   ├── mod.rs
│   │   ├── app_state.rs       # Application state shared via Axum's State extractor
│   │   └── task_model.rs      # Task model and controller logic
│   ├── controllers/
│   │   ├── mod.rs
│   │   └── task_controllers.rs # Request handler functions (create, read, update, delete)
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── app_routes.rs      # Main application router
│   │   └── task_routes.rs     # Task-related routes
├── tests/
│   └── quick_dev.rs           # Integration tests using `httpc-test`
```

---

## 🚀 Getting Started

### Prerequisites
- Rust (Stable) - https://rustup.rs
- Cargo (comes with Rust)

### Running the Server
```bash
cargo run
```

### Running the Server with Watch Mode (watch for changes in `src/`)
```bash
cargo watch -q -c -w src/ -x run
```

### Running the Tests (watch for changes in `tests/`)
```bash
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

---

## ⚙️ Features

- Create a task
- Get all tasks
- Get a single task by ID
- Update a task
- Delete a task

Each task includes:
- `id`: `u64` (auto-incremented)
- `title`: `String`
- `description`: `String`
- `status`: `String`

---

## 🔗 API Endpoints

Base URL: `http://127.0.0.1:8010/api/task`

### Create Task
```
POST /api/task
Content-Type: application/json
{
  "title": "Task title",
  "description": "Task description",
  "status": "pending"
}
```

### Get All Tasks
```
GET /api/task
```

### Get Task By ID
```
GET /api/task/{id}
```

### Update Task
```
PATCH /api/task/{id}
Content-Type: application/json
{
  "title": "Updated title",
  "description": "Updated description",
  "status": "done"
}
```

### Delete Task
```
DELETE /api/task/{id}
```

---

## 🔜 Design Overview

### State Management
The `TaskController` holds the internal task store as:
```rust
task: Arc<Mutex<Vec<Option<Task>>>>
```
- Tasks are stored in a vector.
- `Option<Task>` allows for soft deletions.
- Shared across handlers via `AppState`.

### Concurrency
The store is protected using `Arc<Mutex<>>` to allow safe mutable access across async handlers.

### Error Handling
Custom `Error` enum implements `IntoResponse` to standardize error responses.

---

## 📚 Testing

The integration tests use the `httpc-test` crate and are located in the `tests/` directory.

Example:
```rust
let hc = httpc_test::new_client("http://localhost:8010")?;

hc.do_post("/api/task", json!({ "title": "task" })).await?.print().await?;
```

To run tests automatically when `tests/` changes:
```bash
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

---

## 📊 Future Improvements

- Add persistent storage (PostgreSQL, SQLite)
- Add user authentication (JWT)
- Paginate `get_all_tasks`
- Add validation via `validator` crate
- Implement logging and tracing

---

## 🙌 Credits
- Built with [Axum](https://docs.rs/axum)
- Async runtime powered by [Tokio](https://tokio.rs)
- Testing via [httpc-test](https://crates.io/crates/httpc-test)

---

## 📝 License
This project is open-source and available under the [MIT License](LICENSE).

