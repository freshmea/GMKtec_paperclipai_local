
pub struct SharedStore {
    data: String,
}

impl SharedStore {
    pub fn new(initial_data: &str) -> Self {
        SharedStore {
            data: initial_data.to_string(),
        }
    }

    pub fn read(&self) -> &str {
        &self.data
    }

    pub fn update(&mut self, new_data: &str) {
        self.data = new_data.to_string();
    }
}
