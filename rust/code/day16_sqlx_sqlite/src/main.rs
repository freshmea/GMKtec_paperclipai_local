use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    // In a real scenario, DATABASE_URL would be in .env
    let database_url = "sqlite::memory:";
    let pool = SqlitePool::connect(database_url).await?;

    // Create table for demonstration (In real apps, use migrations)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE)",
    )
    .execute(&pool)
    .await?;

    let shared_state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_users(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

async fn get_user(
    Path(id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}
