use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTask {
    completed: bool,
}

struct AppState {
    tasks: Mutex<Vec<Task>>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState {
        tasks: Mutex::new(Vec::new()),
    });

    let app = Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).put(update_task))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_tasks(State(state): State<Arc<AppState>>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    Json(tasks.clone())
}

async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let mut tasks = state.tasks.lock().unwrap();
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let new_task = Task {
        id: new_id,
        title: payload.title,
        completed: false,
    };
    tasks.push(new_task.clone());
    (StatusCode::CREATED, Json(new_task))
}

async fn get_task(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();
    tasks
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_task(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    let mut tasks = state.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = payload.completed;
        Ok(Json(task.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
