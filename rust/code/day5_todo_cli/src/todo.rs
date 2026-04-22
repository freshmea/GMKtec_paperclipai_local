
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, task: &str) -> Self {
        Todo {
            id,
            task: task.to_string(),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}
