# Day 2. FizzBuzz 및 숫자 맞히기 실습

이 프로젝트는 Day 2의 실습 및 랩(Lab) 과제를 위한 가이드입니다.

## 1. FizzBuzz 구현

`FizzBuzz`는 1부터 100까지 숫자를 출력하되, 3의 배수이면 "Fizz", 5의 배수이면 "Buzz", 3과 5의 공배수이면 "FizzBuzz"를 출력하는 문제입니다.

### 구현 코드 (`src/main.rs`)

```rust
fn main() {
    for i in 1..=100 {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{}", i),
        }
    }
}
```

## 2. 숫자 맞히기 게임 (Number Guessing Game)

사용자가 컴퓨터가 생각한 숫자를 맞히는 게임입니다.

### 구현 단계

1.  **프로젝트 생성:** `cargo new guessing_game`
2.  **의존성 추가:** `std::io`를 사용하여 사용자 입력을 받습니다.
3.  **핵심 로직:**
    - `rand` 크레이트를 사용하여 난수를 생성합니다. (실제 강의에서는 `rand` 설치 과정을 포함할 수 있습니다.)
    - `loop`를 사용하여 정답을 맞힐 때까지 반복합니다.
    - `match`를 사용하여 입력값이 숫자인지 확인하고 비교합니다.

### 예제 코드 (`src/main.rs`)

```rust
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("숫자를 맞춰보세요! (1~100)");

    let secret_number = 42; // 데모를 위해 고정된 숫자 사용

    loop {
        println!("입력해주세요:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("입력 실패");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("숫자를 입력해주세요!");
                continue;
            }
        };

        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("너무 작아요!"),
            Ordering::Greater => println!("너무 커요!"),
            Ordering::Equal => {
                println!("정답입니다!");
                break;
            }
        }
    }
}
```

## 3. 학습 포인트
- `match` 표현식을 활용한 깔끔한 분기 처리.
- `loop`, `break`, `continue`를 이용한 제어 흐름 제어.
- `Result` 타입과 `match`를 이용한 에러 처리(Parsing).
- `std::cmp::Ordering`을 통한 비교 로직.
