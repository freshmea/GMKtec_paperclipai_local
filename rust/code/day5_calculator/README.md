# Day 5 실습 가이드: 강화된 계산기 (Enhanced Calculator)

이 가이드는 1주차의 마무리로, 지금까지 배운 변수, 타입, 함수, 제어 흐름, 그리고 모듈 시스템을 활용하여 더 강력한 계산기를 만드는 것을 목표로 합니다.

## 1. 목표
- 모듈 시스템을 사용하여 코드를 논리적으로 분리한다.
- `match` 문을 사용하여 다양한 연산자를 처리한다.
- `std::io`를 사용하여 사용자 입력을 받는다.
- 함수를 통해 기능을 모듈화한다.

## 2. 프로젝트 구조
```text
day5_calculator/
├── Cargo.toml
└── src/
    ├── main.rs
    └── calculator.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
터미널에서 다음 명령어를 실행하여 새 프로젝트를 생성하세요.
```bash
cargo new day5_calculator
cd day5_calculator
```

### 단계 2: 계산 로직 구현 (`src/calculator.rs`)
`src/calculator.rs` 파일을 생성하고 다음 내용을 작성하세요. 계산 로직을 별도의 모듈로 분리합니다.

```rust
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Square,
}

pub fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err("0으로 나눌 수 없습니다.".to_string())
            } else {
                Ok(a / b)
            }
        }
        Operation::Square => Ok(a * a),
    }
}
```

### 단계 3: 메인 프로그램 구현 (`src/main.rs`)
`src/main.rs`에서 사용자 입력을 받고 `calculator` 모듈을 호출합니다.

```rust
mod calculator;

use std::io::{self, Write};
use calculator::{calculate, Operation};

fn main() {
    println!("--- 강화된 Rust 계산기 ---");

    let mut input = String::new();
    print!("첫 번째 숫자 입력: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().expect("숫자를 입력하세요");

    input.clear();
    print!("연산자 선택 (+, -, *, /, ^): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let op_char = input.trim();

    let op = match op_char {
        "+" => Some(Operation::Add),
        "-" => Some(Operation::Subtract),
        "*" => Some(Operation::Multiply),
        "/" => Some(Operation::Divide),
        "^" => Some(Operation::Square),
        _ => None,
    };

    if let Some(operation) = op {
        let mut num2 = 0.0;
        if !matches!(operation, Operation::Square) {
            input.clear();
            print!("두 번째 숫자 입력: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            num2 = input.trim().parse().expect("숫자를 입력하세요");
        }

        match calculate(num1, num2, operation) {
            Ok(result) => println!("결과: {}", result),
            Err(e) => println!("오류: {}", e),
        }
    } else {
        println!("잘못된 연산자입니다.");
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

## 5. 도전 과제 (Extra Credit)
- `Square` 연산뿐만 아니라 `Cube` 연산도 추가해 보세요.
- 사용자가 잘못된 값을 입력했을 때 `panic!` 대신 `Result`를 사용하여 우아하게 에러를 처리하도록 개선해 보세요.
- `match` 문을 더 정교하게 사용하여 모든 에러 케이스를 핸들링하세요.
