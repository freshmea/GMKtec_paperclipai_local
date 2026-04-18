# Day 9 실습 가이드: 도서 관리 시스템 (Library System)

이 가이드는 `struct`와 `enum`을 조합하여 현실 세계의 도메인 모델을 설계하고, 데이터의 구조화와 행위(Method) 정의를 연습하는 것을 목표로 합니다.

## 1. 목표
- `Book`, `Author`, `Library` 등 도메인 엔티티를 `struct`로 설계한다.
- `enum`을 사용하여 도서 상태(Available, Borrowed, Reserved)를 관리한다.
- 구조체 간의 관계(Library가 Book을 소유)를 구현한다.
- 메서드를 통해 도서 대여, 반납, 검색 등의 비즈니스 로직을 구현한다.

## 2. 프로젝트 구조
```text
day9_library_system/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day9_library_system
cd day9_library_system
```

### 단계 2: 도메인 모델 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하세요.

```rust
#[derive(Debug, Clone, PartialEq)]
enum BookStatus {
    Available,
    Borrowed { borrower: String },
    Reserved { user: String },
}

#[derive(Debug, Clone)]
struct Author {
    name: String,
    nationality: String,
}

#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    author: Author,
    status: BookStatus,
}

impl Book {
    fn new(id: u32, title: &str, author: Author) -> Self {
        Book {
            id,
            title: title.to_string(),
            author,
            status: BookStatus::Available,
        }
    }

    // 대여 기능
    fn borrow_book(&mut self, user_name: String) -> Result<(), String> {
        match self.status {
            BookStatus::Available => {
                self.status = BookStatus::Borrowed { borrower: user_name };
                Ok(())
            }
            _ => Err(format!("현재 대여할 수 없는 상태입니다: {:?}", self.status)),
        }
    }

    // 반납 기능
    fn return_book(&mut self) -> Result<(), String> {
        match self.status {
            BookStatus::Borrowed { .. } => {
                self.status = BookStatus::Available;
                Ok(())
            }
            _ => Err("반납할 수 있는 상태가 아닙니다.".to_string()),
        }
    }
}

struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    fn new(name: &str) -> Self {
        Library {
            name: name.to_string(),
            books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn find_book_by_title(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|b| b.title == title)
    }

    fn list_available_books(&self) {
        println!("--- [{}] 대여 가능한 도서 목록 ---", self.name);
        for book in &self.books {
            if let BookStatus::Available = book.status {
                println!("ID: {}, 제목: {}, 저자: {}", book.id, book.title, book.author.name);
            }
        }
    }
}

fn main() {
    let mut my_library = Library::new("Rust 중앙 도서관");

    // 저자 생성
    let author1 = Author {
        name: "Steve Klabnik".to_string(),
        nationality: "USA".to_string(),
    };
    let author2 = Author {
        name: "Carol Nichols".to_string(),
        nationality: "USA".to_string(),
    };

    // 도서 추가
    my_library.add_book(Book::new(1, "The Rust Programming Language", author1.clone()));
    my_library.add_book(Book::new(2, "Programming Rust", author2.clone()));
    my_library.add_book(Book::new(3, "Rust in Action", author2.clone()));

    // 1. 대여 가능 목록 확인
    my_library.list_available_books();

    // 2. 도서 대여 시도
    println!("\n[대여 시도: 'The Rust Programming Language']");
    if let Some(book) = my_library.find_book_by_title("The Rust Programming Language") {
        match book.borrow_book("Alice".to_string()) {
            Ok(_) => println!("Alice가 책을 대여했습니다."),
            Err(e) => println!("대여 실패: {}", e),
        }
    }

    // 3. 대여 후 목록 확인
    println!("\n[대여 후 목록 확인]");
    my_library.list_available_books();

    // 4. 다시 대여 시도 (실패 케이스)
    println!("\n[중복 대여 시도]");
    if let Some(book) = my_library.find_book_by_title("The Rust Programming Language") {
        match book.borrow_book("Bob".to_string()) {
            Ok(_) => println!("Bob이 책을 대여했습니다."),
            Err(e) => println!("대여 실패: {}", e),
        }
    }

    // 5. 반납
    println!("\n[반납 시도]");
    if let Some(book) = my_library.find_book_by_title("The Rust Programming Language") {
        let _ = book.return_book();
        println!("책이 반납되었습니다.");
    }

    // 6. 최종 확인
    my_library.list_available_books();
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **소유권과 클론:** `author1.clone()`을 사용하는 이유를 소유권 관점에서 생각해보세요.
2. **상태 관리:** `BookStatus` enum이 어떻게 도서의 상태를 안전하게 정의하고, `match`를 통해 잘못된 상태 전이(예: 이미 대여 중인 책을 또 대여)를 막는지 확인하세요.
3. **Option 활용:** `find_book_by_title`이 `Option<&Book>`을 반환함으로써, 책이 없을 경우를 어떻게 안전하게 처리하는지 관찰하세요.

## 5. 도전 과제 (Extra Credit)
- **예약 기능:** `Reserved { user: String }` 상태를 활용하여, 대여 가능한 책을 특정 사용자가 예약할 수 있는 기능을 추가해 보세요.
- **도서 검색 확장:** 제목뿐만 아니라 저자 이름으로도 책을 검색할 수 있는 메서드를 `Library`에 추가해 보세요.
- **에러 커스텀:** `String` 대신 사용자 정의 에러 타입(예: `enum LibraryError`)을 만들어 `Result`를 반환하도록 리팩토링해 보세요.
