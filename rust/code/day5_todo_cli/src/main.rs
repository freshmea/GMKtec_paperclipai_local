mod todo;
mod todo_list;

use todo_list::TodoList;
use std::io::{self, Write};

fn main() {
    let mut my_list = TodoList::new();

    loop {
        println!("\n--- TODO CLI (v1) ---");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Quit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                my_list.add_task(task.trim());
                println!("Task added!");
            }
            "2" => my_list.list_tasks(),
            "3" => {
                print!("Enter task ID to complete: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                if let Ok(id) = id_str.trim().parse::<u32>() {
                    if let Err(e) = my_list.complete_task(id) {
                        println!("Error: {}", e);
                    } else {
                        println!("Task {} completed!", id);
                    }
                } else {
                    println!("Invalid ID format.");
                }
            }
            "4" => break,
            _ => println!("Invalid option."),
        }
    }
    println!("Goodbye!");
}
