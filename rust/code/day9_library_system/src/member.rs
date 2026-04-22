#[derive(Debug, Clone)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub borrowed_books: Vec<String>,
}

impl Member {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            borrowed_books: Vec::new(),
        }
    }
}
