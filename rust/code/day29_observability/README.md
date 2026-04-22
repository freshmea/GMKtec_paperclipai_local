# Day 29: Observability (관측 가능성)

## Objective
Understand the core concepts of Observability (Metrics, Logging, Tracing) and learn how to use the `tracing` crate to implement structured logging and span management in Rust.

## Key Concepts
- **Observability Pillars**:
    - **Logs**: Discrete events recorded with context.
    - **Metrics**: Aggregated numerical data over time.
    - **Traces**: The end-to-end journey of a request across different components.
- **Spans**: A period of time during which an operation is being performed, providing context to all events within that period.
- **Events**: A single point in time within a span that records a specific occurrence.
- **Structured Logging**: Attaching key-value pairs (fields) to logs to make them machine-readable and searchable.

## Lab Instructions
1. Open `src/main.rs`.
2. Observe how `span!` is used to create a logical scope for `request_processing` and `database_query`.
3. Notice how `request_id` is attached as a field to the span, and how it persists through nested spans.
4. Observe the difference between `info!` and `error!` logs and how they appear in the output.
5. Run the code: `cargo run`.
6. **Challenge**: Modify the code to use the `#[instrument]` macro instead of manual `span!` creation for the `process_request` function.

## Running the Code
```bash
cargo run
```

## Testing
```bash
cargo test
```
