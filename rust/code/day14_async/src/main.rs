use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

// --- 1. Async/Await Basics ---
async fn async_hello(name: &str) {
    sleep(Duration::from_millis(500)).await;
    println!("Hello, {}! (after 500ms)", name);
}

// --- 2. Tokio Spawn & Join ---
async fn spawn_tasks() {
    println!("\n--- 2. Tokio Spawn & Join ---");
    let mut handles = vec![];

    for i in 0..5 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(i * 100)).await;
            format!("Task {} completed", i)
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("Task failed: {:?}", e),
        }
    }
}

// --- 3. MPSC in Async ---
async fn async_channel() {
    println!("\n--- 3. Async MPSC Channel ---");
    let (tx, mut rx) = mpsc::channel(32);

    // Producer task
    tokio::spawn(async move {
        for i in 1..=5 {
            let msg = format!("Message {}", i);
            if let Err(e) = tx.send(msg).await {
                eprintln!("Send error: {}", e);
                break;
            }
            sleep(Duration::from_millis(200)).await;
        }
    });

    // Consumer
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}

// --- 4. Tokio Select (Timeout pattern) ---
async fn select_timeout() {
    println!("\n--- 4. Tokio Select (Timeout pattern) ---");
    
    let slow_task = async {
        sleep(Duration::from_secs(3)).await;
        "Slow task finished"
    };

    tokio::select! {
        res = slow_task => println!("Result: {}", res),
        _ = sleep(Duration::from_secs(1)) => println!("Error: Task timed out!"),
    }
}

// --- 5. Shared State in Async (Arc + Mutex) ---
async fn shared_state_async() {
    println!("\n--- 5. Shared State (Arc + tokio::sync::Mutex) ---");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut num = counter_clone.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    println!("Final Counter: {}", *counter.lock().await);
}

#[tokio::main]
async fn main() {
    println!("--- 1. Async/Await Basics ---");
    async_hello("Rustacean").await;

    spawn_tasks().await;
    async_channel().await;
    select_timeout().await;
    shared_state_async().await;
}
