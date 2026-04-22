mod processor;
mod shared_store;

use processor::DataProcessor;
use shared_store::SharedStore;

fn main() {
    // 1. Ownership & Move
    println!("--- 1. Ownership & Move ---");
    let s1 = String::from("Hello");
    let s2 = s1; // s1 moved to s2
    println!("s2: {}", s2);
    // println!("{}", s1); // Error: use of moved value

    // 2. Borrowing (Immutable)
    println!("\n--- 2. Immutable Borrowing ---");
    let s3 = String::from("Rust");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {}, r2: {}", r1, r2);

    // 3. Borrowing (Mutable)
    println!("\n--- 3. Mutable Borrowing ---");
    let mut s4 = String::from("Mutable");
    {
        let r3 = &mut s4;
        r3.push_str(" String");
        println!("r3: {}", r3);
    } // r3 goes out of scope
    println!("s4: {}", s4);

    // 4. Data Processor (Slices & Lifetimes)
    println!("\n--- 4. Data Processor (Slices & Lifetimes) ---");
    let words = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
    ];
    let filtered = DataProcessor::filter_pattern(&words, "a");
    println!("Words containing 'a': {:?}", filtered);

    // 5. Shared Store
    println!("\n--- 5. Shared Store ---");
    let mut store = SharedStore::new("Initial State");
    println!("Store: {}", store.read());
    store.update("New State");
    println!("Store: {}", store.read());
}
