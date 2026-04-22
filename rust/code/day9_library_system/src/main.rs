use library_system::{Book, BookStatus, LibraryManager, Member};

fn main() {
    let mut library = LibraryManager::new();

    // 1. Setup data
    let book1 = Book::new("B001", "The Rust Programming Language", "Steve Klabnik");
    let book2 = Book::new("B002", "Programming Rust", "Jim Blandy");
    
    let member1 = Member::new("M001", "Alice");

    library.add_book(book1);
    library.add_book(book2);
    library.add_member(member1);

    println!("--- Initial State ---");
    println!("Available books: {:?}", library.list_available_books().iter().map(|b| &b.title).collect::<Vec<_>>());

    // 2. Borrow a book
    println!("\n--- Borrowing Book ---");
    match library.borrow_book("B001", "M001") {
        Ok(_) => println!("Alice borrowed B001 successfully."),
        Err(e) => println!("Error borrowing B001: {:?}", e),
    }

    println!("Available books: {:?}", library.list_available_books().iter().map(|b| &b.title).collect::<Vec<_>>());

    // 3. Try to borrow the same book again
    println!("\n--- Attempting to borrow B001 again ---");
    match library.borrow_book("B001", "M001") {
        Ok(_) => println!("Borrowed successfully (should not happen!)"),
        Err(e) => println!("Expected Error: {:?}", e),
    }

    // 4. Return the book
    println!("\n--- Returning Book ---");
    match library.return_book("B001") {
        Ok(_) => println!("Alice returned B001 successfully."),
        Err(e) => println!("Error returning B001: {:?}", e),
    }

    println!("Available books: {:?}", library.list_available_books().iter().map(|b| &b.title).collect::<Vec<_>>());
}
