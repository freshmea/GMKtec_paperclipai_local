use axum::{
    routing::{get, post},
    extract::{Path, State},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{info, instrument};
use uuid::Uuid;

// --- Data Models ---

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: Uuid,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTask {
    title: String,
}

// --- Shared State ---

type Db = Arc<RwLock<HashMap<Uuid, Task>>>;

// --- Handlers ---

#[instrument]
async fn list_tasks(State(db): State<Db>) -> Json<Vec<Task>> {
    let tasks = db.read().unwrap().values().cloned().collect();
    info!("Listed {} tasks", tasks.len());
    Json(tasks)
}

#[instrument]
async fn create_task(
    State(db): State<Db>,
    Json(payload): Json<CreateTask>,
) -> Json<Task> {
    let task = Task {
        id: Uuid::new_v4(),
        title: payload.title,
        completed: false,
    };
    db.write().unwrap().insert(task.id, task.clone());
    info!(task_id = %task.id, "Created new task");
    Json(task)
}

#[instrument]
async fn get_task(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
) -> Result<Json<Task>, axum::http::StatusCode> {
    let tasks = db.read().unwrap();
    if let Some(task) = tasks.get(&id) {
        info!(task_id = %id, "Fetched task");
        Ok(Json(task.clone()))
    } else {
        info!(task_id = %id, "Task not found");
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

#[instrument]
async fn delete_task(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
) -> axum::http::StatusCode {
    if db.write().unwrap().remove(&id).is_some() {
        info!(task_id = %id, "Deleted task");
        axum::http::StatusCode::NO_CONTENT
    } else {
        info!(task_id = %id, "Failed to delete: Task not found");
        axum::http::StatusCode::NOT_FOUND
    }
}

// --- Main Application ---

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Final Project: Task Management API");

    let db: Db = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).delete(delete_task))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}
