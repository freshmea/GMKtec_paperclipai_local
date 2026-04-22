use std::rc::Rc;

#[derive(Debug)]
struct SharedData {
    content: String,
}

fn main() {
    // --- 1. Basic Rc Usage ---
    println!("--- 1. Basic Rc Usage ---");
    
    // Create the initial owner
    let data = Rc::new(SharedData {
        content: String::from("Hello, Shared World!"),
    });
    println!("Initial data: {:?}", data);
    println!("Reference count: {}", Rc::strong_count(&data));

    // Create multiple owners using Rc::clone
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);
    let owner3 = Rc::clone(&data);

    println!("After cloning to owner1, owner2, and owner3...");
    println!("Reference count: {}", Rc::strong_count(&data));

    // Accessing data through different owners
    println!("Owner 1 sees: {}", owner1.content);
    println!("Owner 2 sees: {}", owner2.content);
    println!("Owner 3 sees: {}", owner3.content);

    // --- 2. Drop and Reference Counting ---
    println!("\n--- 2. Drop and Reference Counting ---");
    
    println!("Dropping owner3...");
    drop(owner3);
    println!("Reference count after dropping owner3: {}", Rc::strong_count(&data));

    println!("Dropping owner2...");
    drop(owner2);
    println!("Reference count after dropping owner2: {}", Rc::strong_count(&data));

    println!("Dropping owner1...");
    drop(owner1);
    // Note: 'data' is still in scope, so the count should be 1
    println!("Reference count after dropping owner1 (but 'data' still exists): {}", Rc::strong_count(&data));

    // --- 3. Immutability Observation ---
    println!("\n--- 3. Immutability Observation ---");
    println!("Note: You cannot modify 'data.content' directly through an Rc pointer.");
    println!("To allow mutation, you would need Rc<RefCell<T>> (covered in the next session).");
    
    // Uncommenting the following line would cause a compilation error:
    // data.content = String::from("Changed!"); 
}
