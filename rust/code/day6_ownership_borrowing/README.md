# Day 6 실습 가이드: 소유권과 빌림 (Ownership and Borrowing)

이 가이드는 Rust의 핵심인 소유권(Ownership)과 빌림(Borrowing) 개념을 코드로 직접 확인하며 익히는 것을 목표로 합니다. 컴파일 에러를 두려워하지 말고, 컴파일러가 왜 에러를 내는지 이해하는 것이 핵심입니다.

## 1. 목표
- `Move`와 `Copy`의 차이점 이해하기.
- 불변 참조(`&T`)와 가변 참조(`&mut T`)의 규칙 익히기.
- 빌림 검사기(Borrow Checker)의 에러 메시지 해석하기.

## 2. 프로젝트 구조
```text
day6_ownership_borrowing/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day6_ownership_borrowing
cd day6_ownership_borrowing
```

### 단계 2: 소유권과 복사 실습 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 주석 처리된 부분을 하나씩 해제하며 컴파일 에러를 관찰하세요.

```rust
fn main() {
    // --- 1. Move vs Copy ---
    println!("--- 1. Move vs Copy ---");
    
    let s1 = String::from("hello");
    // let s2 = s1; // [실습] 이 주석을 해제해보세요. s1은 더 이상 사용할 수 없게 됩니다 (Move).
    // println!("s1: {}, s2: {}", s1, s2); 

    let x = 5;
    let y = x; // [실습] 정수(i32)는 Copy trait를 구현하므로 복사됩니다.
    println!("x: {}, y: {}", x, y); // 둘 다 사용 가능합니다.

    // --- 2. Immutable Borrowing (&) ---
    println!("\n--- 2. Immutable Borrowing ---");
    let s3 = String::from("rust");
    let len = calculate_length(&s3); // 소유권을 넘기지 않고 빌려줌
    println!("'{}'의 길이는 {}입니다.", s3, len); // s3는 여전히 사용 가능

    // --- 3. Mutable Borrowing (&mut) ---
    println!("\n--- 3. Mutable Borrowing ---");
    let mut s4 = String::from("hello");
    change(&mut s4); // 가변 참조를 전달
    println!("수정된 문자열: {}", s4);

    // --- 4. The Borrow Checker Error (의도적 에러) ---
    println!("\n--- 4. Borrow Checker Error (실습용) ---");
    let mut s5 = String::from("error");
    let r1 = &s5; // 불변 참조
    let r2 = &s5; // 또 다른 불변 참조 (OK)
    println!("r1: {}, r2: {}", r1, r2);

    // let r3 = &mut s5; // [실습] 이 주석을 해제해보세요. 
    // 불변 참조(r1, r2)가 살아있는 동안 가변 참조(r3)를 만들 수 없습니다.
    // println!("r3: {}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 가이드:**
1. `let s2 = s1;`의 주석을 풀고 `cargo check`를 실행하여 `value borrowed here after move` 에러를 확인하세요.
2. `let r3 = &mut s5;`의 주석을 풀고 컴파일러가 왜 "cannot borrow `s5` as mutable because it is also borrowed as immutable"라고 말하는지 읽어보세요.

## 5. 도전 과제 (Extra Credit)
- `calculate_length` 함수의 인자 타입을 `&String`에서 `&str`로 변경해 보세요. 왜 이것이 더 범용적인지(Generic한지) 고민해 보세요.
- `change` 함수를 호출하기 전에 `r1`이나 `r2`를 사용하는 코드를 배치하여 에러를 유도해 보세요.
