# Day 1. 계산기 (Calculator) 프로젝트

이 프로젝트는 Day 1의 마지막 랩(Lab) 과제입니다. 사칙연산을 수행하는 간단한 계산기를 만듭니다.

## 1. 프로젝트 구조

```text
day1_calculator/
├── Cargo.toml
└── src/
    └── main.rs
```

## 2. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day1_calculator
cd day1_calculator
```

### 단계 2: 기본 코드 작성
`src/main.rs`에 다음 코드를 작성하여 사칙연산 로직을 구현합니다.

```rust
fn main() {
    let num1 = 10.0;
    let num2 = 5.0;
    let operator = '+';

    let result = calculate(num1, num2, operator);
    println!("{} {} {} = {}", num1, operator, num2, result);
}

fn calculate(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero!");
                0.0
            }
        },
        _ => {
            println!("Error: Invalid operator!");
            0.0
        }
    }
}
```

### 단계 3: 실행 및 검증
```bash
cargo run
cargo check
cargo fmt
```

## 3. 학습 포인트
- `f64` 타입을 사용하여 정수와 실수 계산 모두 지원하기.
- `match` 문을 사용하여 다양한 연산자 처리하기.
- `match`의 와일드카드(`_`)를 사용하여 예외 케이스 처리하기.
