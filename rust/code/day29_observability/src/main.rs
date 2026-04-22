use tracing::{info, error, span, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

async fn process_request(request_id: Uuid) {
    // Create a span for the entire request processing
    let request_span = span!(Level::INFO, "request_processing", request_id = %request_id);
    let _enter = request_span.enter();

    info!("Starting request processing...");

    // Simulate some work
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Create a nested span for a specific sub-task
    {
        let sub_task_span = span!(Level::INFO, "database_query");
        let _enter_sub = sub_task_span.enter();
        
        info!("Executing database query...");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        info!("Database query completed.");
    }

    info!("Request processing finished successfully.");
}

async fn handle_error_scenario(request_id: Uuid) {
    let request_span = span!(Level::ERROR, "request_processing_error", request_id = %request_id);
    let _enter = request_span.enter();

    info!("Starting a request that will fail...");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    error!("An unexpected error occurred during processing!");
}

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Observability Demo Started");

    // Scenario 1: Successful request
    let id1 = Uuid::new_v4();
    process_request(id1).await;

    // Scenario 2: Error request
    let id2 = Uuid::new_v4();
    handle_error_scenario(id2).await;

    info!("Observability Demo Finished");
}
