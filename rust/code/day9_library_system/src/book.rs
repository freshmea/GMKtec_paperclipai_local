#[derive(Debug, Clone)]
pub enum BookStatus {
    Available,
    Borrowed { borrowed_by: String },
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

impl Book {
    pub fn new(id: &str, title: &str, author: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            status: BookStatus::Available,
        }
    }
}
