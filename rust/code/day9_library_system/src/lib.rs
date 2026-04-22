mod book;
mod member;

pub use book::{Book, BookStatus};
pub use member::Member;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum LibraryError {
    BookNotFound,
    MemberNotFound,
    BookAlreadyBorrowed,
    BookNotBorrowed,
    MemberLimitReached,
}

pub struct LibraryManager {
    books: HashMap<String, Book>,
    members: HashMap<String, Member>,
}

impl LibraryManager {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            members: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.id.clone(), book);
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.insert(member.id.clone(), member);
    }

    pub fn borrow_book(&mut self, book_id: &str, member_id: &str) -> Result<(), LibraryError> {
        let book = self.books.get_mut(book_id).ok_or(LibraryError::BookNotFound)?;
        let member = self.members.get_mut(member_id).ok_or(LibraryError::MemberNotFound)?;

        if let BookStatus::Borrowed { borrowed_by } = &book.status {
            if borrowed_by == member_id {
                return Err(LibraryError::BookAlreadyBorrowed);
            }
            return Err(LibraryError::BookAlreadyBorrowed);
        }

        book.status = BookStatus::Borrowed {
            borrowed_by: member_id.to_string(),
        };
        
        member.borrowed_books.push(book_id.to_string());
        Ok(())
    }

    pub fn return_book(&mut self, book_id: &str) -> Result<(), LibraryError> {
        let book = self.books.get_mut(book_id).ok_or(LibraryError::BookNotFound)?;

        if let BookStatus::Borrowed { borrowed_by } = &book.status {
            let member_id = borrowed_by.clone();
            book.status = BookStatus::Available;

            if let Some(member) = self.members.get_mut(&member_id) {
                member.borrowed_books.retain(|id| id != book_id);
            }
            Ok(())
        } else {
            Err(LibraryError::BookNotBorrowed)
        }
    }

    pub fn list_available_books(&self) -> Vec<&Book> {
        self.books
            .values()
            .filter(|b| matches!(b.status, BookStatus::Available))
            .collect()
    }
}
