use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

// --- 1. Threads: Parallel Computation ---
fn parallel_squares(numbers: Vec<i32>) -> Vec<i32> {
    let mut handles = vec![];

    for num in numbers {
        let handle = thread::spawn(move || {
            // Simulate some work
            thread::sleep(Duration::from_millis(10));
            num * num
        });
        handles.push(handle);
    }

    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}

// --- 2. Channels: Message Passing Pipeline ---
fn channel_pipeline() -> i32 {
    let (tx, rx) = mpsc::channel();

    // Spawn 3 producers
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let val = i * 10;
            println!("Producer {}: sending {}", i, val);
            tx_clone.send(val).unwrap();
        });
    }

    // Drop the original sender so rx knows when to stop
    drop(tx);

    let mut sum = 0;
    for received in rx {
        println!("Received: {}", received);
        sum += received;
    }
    sum
}

// --- 3. Arc & Mutex: Shared State ---
fn shared_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    result
}

fn main() {
    println!("--- 1. Threads: Parallel Squares ---");
    let nums = vec![1, 2, 3, 4, 5];
    let squares = parallel_squares(nums);
    println!("Squares: {:?}", squares);

    println!("\n--- 2. Channels: Message Passing ---");
    let total_sum = channel_pipeline();
    println!("Total Sum from Channels: {}", total_sum);

    println!("\n--- 3. Arc & Mutex: Shared Counter ---");
    let final_count = shared_counter();
    println!("Final Shared Counter: {}", final_count);
}
