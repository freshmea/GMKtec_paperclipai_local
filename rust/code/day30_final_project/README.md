# Day 30: Final Project (최종 프로젝트)

## Objective
Integrate all learned concepts (Ownership, Concurrency, Async, Error Handling, Web, Observability) to design and implement a production-level application.

## Project Overview
This project is a **Task Management REST API** built with:
- **Axum**: Web framework for routing and handling requests.
- **Tokio**: Asynchronous runtime for high-performance I/O.
- **Serde**: Serialization/Deserialization for JSON payloads.
- **Tracing**: Structured logging and observability.
- **Shared State**: Thread-safe in-memory storage using `Arc<RwLock<T>>`.

## Key Features
- `GET /tasks`: List all tasks.
- `POST /tasks`: Create a new task.
- `GET /tasks/:id`: Retrieve a specific task by ID.
- `DELETE /tasks/:id`: Remove a task from the system.

## Key Concepts Implemented
- **Concurrency & Shared State**: Using `Arc` for shared ownership and `RwLock` for thread-safe interior mutability.
- **Asynchronous Programming**: Using `async/await` for non-blocking web server operations.
- **Structured Logging**: Using `tracing` and `#[instrument]` to provide context for every API call.
- **Error Handling**: Using Axum's `Result` pattern with HTTP status codes.

## Lab Instructions
1. Open `src/main.rs`.
2. Examine how the `Db` type is defined using `Arc<RwLock<...>>` to allow safe access across multiple threads.
3. Observe the use of `#[instrument]` on handlers to automatically create spans for each request.
4. Run the server: `cargo run`.
5. **Challenge**: Implement a `PUT /tasks/:id` endpoint to update the `completed` status of an existing task.

## Running the Code
```bash
cargo run
```

## Testing
You can test the API using `curl`:
```bash
# Create a task
curl -X POST http://127.0.0.1:3000/tasks -H "Content-Type: application/json" -d '{"title": "Complete Rust Course"}'

# List tasks
curl http://127.0.0.1:3000/tasks
```
