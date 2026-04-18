# Day 11 실습 가이드: Option과 Result (Option & Result)

이 가이드는 Rust의 핵심 에러 처리 메커니즘인 `Option<T>`와 `Result<T, E>`를 배우고, 이를 통해 `null`이나 예외(Exception) 없이 안전하게 데이터의 존재 여부와 작업의 성공/실패를 다루는 방법을 익히는 것을 목표로 합니다.

## 1. 목표
- `Option<T>`를 사용하여 값이 없을 수 있는 상황을 안전하게 처리한다.
- `Result<T, E>`를 사용하여 작업의 성공과 실패를 명시적으로 다룬다.
- `match`, `if let`, `unwrap`, `expect` 등 다양한 패턴을 통해 값을 추출하는 법을 익힌다.

## 2. 프로젝트 구조
```text
day11_option_result/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day11_option_result
cd day11_option_result
```

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
fn main() {
    // --- 1. Option<T>: 값이 있을 수도, 없을 수도 있는 경우 ---
    println!("--- 1. Option<T> ---");
    let maybe_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // 패턴 매칭을 이용한 안전한 추출
    match maybe_number {
        Some(n) => println!("찾은 숫자: {}", n),
        None => println!("숫자를 찾지 못했습니다."),
    }

    // if let을 이용한 간결한 처리
    if let Some(n) = no_number {
        println!("숫자 발견: {}", n);
    } else {
        println!("no_number는 None입니다.");
    }

    // unwrap()과 expect() - 주의해서 사용!
    // let val = maybe_number.unwrap(); // 성공 시 Some(42) 반환
    // let val2 = no_number.expect("숫자가 반드시 있어야 합니다!"); // None이면 Panic 발생!

    // --- 2. Result<T, E>: 성공과 실패의 처리 ---
    println!("\n--- 2. Result<T, E> ---");
    
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("0으로 나눌 수 없습니다."))
        } else {
            Ok(numerator / denominator)
        }
    }

    let success = divide(10.0, 2.0);
    let failure = divide(10.0, 0.0);

    // Result 처리
    match success {
        Ok(val) => println!("나눗셈 성공: {}", val),
        Err(e) => println!("나눗셈 실패: {}", e),
    }

    match failure {
        Ok(val) => println!("나눗셈 성공: {}", val),
        Err(e) => println!("나눗셈 실패: {}", e),
    }

    // --- 3. Combinators: map과 unwrap_or ---
    println!("\n--- 3. Combinators ---");
    let opt_val = Some(5);
    let mapped = opt_val.map(|x| x * 2); // Some(10)
    let unwrapped = no_number.unwrap_or(0); // None이면 0 반환

    println!("Mapped: {:?}", mapped);
    println!("Unwrapped: {}", unwrapped);
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. `no_number.unwrap()`을 실행했을 때 프로그램이 어떻게 반응하는지 확인하세요 (Panic 발생).
2. `match` 문을 사용하여 `Some`과 `None`을 처리할 때, 모든 경우의 수(arm)를 다 다루었는지 확인하세요 (컴파일러가 체크해줍니다).
3. `unwrap_or`를 사용하여 `None`일 때의 기본값을 설정하는 것이 왜 `unwrap`보다 안전한지 생각해보세요.

## 5. 도전 과제 (Extra Credit)
- `Option`에 `and_then`을 사용하여 연속적인 `Option` 연산을 수행해 보세요.
- `Result`의 `map_err`를 사용하여 에러 메시지를 가공해 보세요.
- `match` 대신 `if let`만 사용하여 모든 코드를 다시 작성해 보세요.
