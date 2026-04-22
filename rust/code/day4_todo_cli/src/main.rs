mod todo;
mod todo_list;

use todo_list::TodoList;

fn main() {
    let mut my_list = TodoList::new();

    println!("Adding tasks...");
    my_list.add_task("Learn Rust Ownership");
    my_list.add_task("Master Traits");
    my_list.add_task("Build a CLI Tool");

    my_list.list_tasks();

    println!("\nCompleting task 2...");
    if let Err(e) = my_list.complete_task(2) {
        println!("Error: {}", e);
    }

    my_list.list_tasks();

    println!("\nAttempting to complete non-existent task 99...");
    if let Err(e) = my_list.complete_task(99) {
        println!("Expected Error: {}", e);
    }
}
