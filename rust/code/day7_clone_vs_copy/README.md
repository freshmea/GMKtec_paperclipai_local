# Day 7 실습 가이드: 깊은 복사(Deep Copy) vs 얕은 복사(Shallow Copy)

이 가이드는 Rust에서 데이터가 복사될 때 메모리 상에서 실제로 어떤 일이 일어나는지, 그리고 `Clone`과 `Copy` 트레이트의 차이점을 이해하는 것을 목표로 합니다.

## 1. 목표
- `Copy` 트레이트가 적용되는 타입(스택 데이터)과 그렇지 않은 타입(힙 데이터)의 차이 이해.
- `Clone`을 통한 명시적인 깊은 복사(Deep Copy) 과정 확인.
- 데이터 이동(Move) 후의 메모리 상태 이해.

## 2. 프로젝트 구조
```text
day7_clone_vs_copy/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day7_clone_vs_copy
cd day7_clone_vs_copy
```

### 단계 2: 복사 동작 실습 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
fn main() {
    // --- 1. Copy 트레이트 (스택 데이터) ---
    println!("--- 1. Copy Trait (Stack Data) ---");
    let x = 42;
    let y = x; // x의 값이 y로 '복사'됩니다.
    
    println!("x: {}, y: {}", x, y); 
    // x와 y는 각각 독립적인 스택 공간을 가지므로 둘 다 사용 가능합니다.

    // --- 2. Move Semantics (힙 데이터) ---
    println!("\n--- 2. Move Semantics (Heap Data) ---");
    let s1 = String::from("hello");
    let s2 = s1; // s1의 소유권이 s2로 '이동(Move)'됩니다.

    // println!("s1: {}", s1); // [실습] 이 주석을 해제해보세요. 컴파일 에러가 발생합니다!
    println!("s2: {}", s2); // s2는 이제 새로운 소유자입니다.

    // --- 3. Clone 트레이트 (깊은 복사) ---
    println!("\n--- 3. Clone Trait (Deep Copy) ---");
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3의 내용을 바탕으로 새로운 힙 메모리를 할당하여 '깊은 복사'를 수행합니다.

    println!("s3: {}, s4: {}", s3, s4); 
    // s3와 s4는 서로 다른 메모리 주소를 가지며, 둘 다 독립적으로 사용 가능합니다.

    // --- 4. 메모리 주소 확인 (실제 주소 비교) ---
    println!("\n--- 4. Memory Address Check ---");
    let s5 = String::from("address_test");
    let s6 = s5.clone();

    println!("s5의 데이터 주소: {:p}", s5.as_ptr());
    println!("s6의 데이터 주소: {:p}", s6.as_ptr());
    // 두 주소가 다르면 서로 다른 힙 메모리를 사용하고 있는 것입니다.
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**확인 사항:**
- `s1`을 사용하려고 했을 때 발생하는 에러 메시지를 읽어보세요.
- `s3.clone()`을 했을 때와 `s1`을 `s2`에 대입했을 때의 결과(사용 가능 여부)가 어떻게 다른지 비교해 보세요.
- 마지막 섹션에서 출력된 두 주소값이 서로 다른지 확인하세요.

## 5. 도전 과제 (Extra Credit)
- `String` 대신 `Vec<i32>`를 사용하여 동일한 실험을 진행해 보세요.
- `Clone`을 사용하지 않고 `s1`의 데이터를 `s2`로 옮기면서도 `s1`을 계속 사용하려면 어떤 트릭(예: `&str` 활용)을 쓸 수 있을지 고민해 보세요.
