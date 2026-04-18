# Day 4. TODO CLI (v1) 프로젝트

이 프로젝트는 Day 4의 랩(Lab) 및 과제를 위한 가이드입니다. 모듈 시스템을 사용하여 구조화된 TODO 리스트 관리 도구를 만듭니다.

## 1. 프로젝트 구조

```text
day4_todo_cli/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    └── todo.rs
```

## 2. 구현 요구 사항

- **`todo.rs` 모듈:**
    - `Todo` 구조체: `id`, `task`, `is_completed` 필드 포함.
    - `TodoList` 구조체: `Vec<Todo>`를 소유하며, 항목 추가, 목록 출력 기능을 가짐.
- **`main.rs`:**
    - 사용자 입력을 받아 `TodoList`의 기능을 호출 (CLI 인터페이스).
- **가시성 제어:** `pub` 키워드를 적절히 사용하여 모듈 간 인터페이스를 설계합니다.

## 3. 예제 코드

### `src/todo.rs`

```rust
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub is_completed: bool,
}

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

    pub fn add(&mut self, task: String) {
        let new_todo = Todo {
            id: self.next_id,
            task,
            is_completed: false,
        };
        self.items.push(new_todo);
        self.next_id += 1;
    }

    pub fn list(&self) {
        if self.items.is_empty() {
            println!("할 일이 없습니다.");
            return;
        }
        for item in &self.items {
            let status = if item.is_completed { "[X]" } else { "[ ]" };
            println!("{} {}: {}", status, item.id, item.task);
        }
    }
}
```

### `src/lib.rs`

```rust
pub mod todo;
```

### `src/main.rs`

```rust
use std::io;
use todo_cli::todo::TodoList;

fn main() {
    let mut my_list = TodoList::new();

    loop {
        println!("\n--- TODO CLI ---");
        println!("1. 추가 (add)");
        println!("2. 목록 (list)");
        println!("3. 종료 (exit)");
        print!("선택: ");
        
        io::stdout().flush().unwrap(); // flush를 위해 std::io::Write 필요

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("입력 실패");

        match choice.trim() {
            "1" => {
                println!("할 일을 입력하세요:");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("입력 실패");
                my_list.add(task.trim().to_string());
            }
            "2" => {
                my_list.list();
            }
            "3" => break,
            _ => println!("잘못된 선택입니다."),
        }
    }
}

// Note: 실제 실행 시 상단에 use std::io::Write; 추가 필요
```
*(참고: 위 코드는 개념 이해를 돕기 위한 요약본입니다. 실제 구현 시 `use std::io::Write;` 등을 적절히 추가해야 합니다.)*

## 4. 학습 포인트

- **모듈 시스템:** `mod`, `use`, `pub`을 이용한 코드 분리.
- **구조체와 구현(`impl`):** 데이터와 행위를 하나로 묶는 법.
- **가시성(Visibility):** `pub`을 통해 외부로 노출할 기능과 내부 데이터(`items`)를 구분하는 법.
- **소유권과 참조:** `TodoList` 내부의 `Vec`을 순회할 때 `&self`를 사용하는 이유.
