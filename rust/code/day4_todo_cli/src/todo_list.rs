
use crate::todo::Todo;

pub struct TodoList {
    items: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, task: &str) {
        let todo = Todo::new(self.next_id, task);
        self.items.push(todo);
        self.next_id += 1;
    }

    pub fn list_tasks(&self) {
        println!("\n--- TODO List ---");
        if self.items.is_empty() {
            println!("No tasks found.");
            return;
        }
        for item in &self.items {
            let status = if item.completed { "[X]" } else { "[ ]" };
            println!("{} {}: {}", status, item.id, item.task);
        }
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(task) = self.items.iter_mut().find(|t| t.id == id) {
            task.complete();
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }
}
