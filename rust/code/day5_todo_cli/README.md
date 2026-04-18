# Day 5 실습 가이드: TODO CLI (v1)

이 가이드는 1주차의 마무리로, 지금까지 배운 변수, 타입, 컬렉션(`Vec`), 문자열, 그리고 제어 흐름을 활용하여 메모리 기반의 간단한 TODO 리스트 도구를 만드는 것을 목표로 합니다.

## 1. 목표
- `Vec<String>`을 사용하여 데이터를 관리한다.
- `match` 문과 `loop`를 사용하여 사용자 인터페이스를 구현한다.
- 문자열 조작 및 입력을 다룬다.

## 2. 프로젝트 구조
```text
day5_todo_cli/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day5_todo_cli
cd day5_todo_cli
```

### 단계 2: 프로그램 구현 (`src/main.rs`)
`src/main.rs` 파일에 다음 코드를 작성하세요. 이 프로그램은 메모리상에 할 일을 저장하고, 추가, 삭제, 목록 보기 기능을 제공합니다.

```rust
use std::io::{self, Write};

fn main() {
    let mut todo_list: Vec<String> = Vec::new();

    println!("--- TODO CLI (v1) ---");
    println!("명령어: add [할일], list, remove [번호], exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 읽기 실패");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];

        match command {
            "add" => {
                if parts.len() < 2 {
                    println!("오류: 추가할 내용을 입력하세요. (예: add 공부하기)");
                } else {
                    let task = parts[1].to_string();
                    todo_list.push(task);
                    println!("추가되었습니다.");
                }
            }
            "list" => {
                if todo_list.is_empty() {
                    println!("할 일이 없습니다.");
                } else {
                    println!("--- 할 일 목록 ---");
                    for (i, task) in todo_list.iter().enumerate() {
                        println!("{}. {}", i + 1, task);
                    }
                }
            }
            "remove" => {
                if parts.len() < 2 {
                    println!("오류: 삭제할 번호를 입력하세요. (예: remove 1)");
                } else {
                    let index_str = parts[1];
                    match index_str.parse::<usize>() {
                        Ok(index) if index > 0 && index <= todo_list.len() => {
                            let removed = todo_list.remove(index - 1);
                            println!("'{}' 항목이 삭제되었습니다.", removed);
                        }
                        _ => println!("오류: 유효한 번호를 입력하세요."),
                    }
                }
            }
            "exit" => {
                println!("프로그램을 종료합니다.");
                break;
            }
            _ => println!("알 수 없는 명령어입니다. (add, list, remove, exit)"),
        }
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**테스트 시나리오:**
1. `add Rust 공부하기` 입력
2. `add 프로젝트 완성하기` 입력
3. `list` 입력하여 목록 확인
4. `remove 1` 입력하여 첫 번째 항목 삭제
5. `list`로 남은 항목 확인
6. `exit`로 종료

## 5. 도전 과제 (Extra Credit)
- **데이터 보존:** 프로그램을 종료해도 데이터가 유지되도록 파일(`txt` 또는 `json`)에 저장하고 불러오는 기능을 추가해 보세요.
- **상태 관리:** 각 항목에 '완료(Done)' 상태를 추가하고, 완료된 항목만 따로 보거나 표시하는 기능을 만들어 보세요.
- **에러 처리:** 숫자가 아닌 값을 입력했을 때의 처리를 더 견고하게 만들어 보세요.
