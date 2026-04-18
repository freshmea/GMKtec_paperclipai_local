# Day 8 실습 가이드: Slice 데모 (Slice Demo)

이 가이드는 Rust의 Slice(`&str`, `&[T]`) 개념을 직접 코드로 확인하며, 데이터의 일부를 어떻게 안전하고 효율적으로 참조하는지 배우는 것을 목표로 합니다.

## 1. 목표
- 문자열 슬라이스(`&str`)를 사용하여 문자열의 부분 집합을 다룬다.
- 배열 슬라이스(`&[T]`)를 사용하여 배열의 일부를 함수로 전달한다.
- 슬라이스가 포인터와 길이로 구성된 '뷰(View)'임을 이해한다.

## 2. 프로젝트 구조
```text
day8_slice_demo/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day8_slice_demo
cd day8_slice_demo
```

### 단계 2: 슬라이스 실습 구현 (`src/main.rs`)
`src/main.rs` 파일에 다음 코드를 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
fn main() {
    // --- 1. String Slice (&str) ---
    println!("--- 1. String Slice ---");
    let s = String::from("Hello, Rust World!");
    
    // 문자열의 특정 범위를 슬라이싱
    let hello = &s[0..5];   // "Hello"
    let rust = &s[7..11];   // "Rust"
    let world = &s[12..17]; // "World"

    println!("Original: {}", s);
    println!("Slice 1: {}", hello);
    println!("Slice 2: {}", rust);
    println!("Slice 3: {}", world);

    // --- 2. Array Slice (&[T]) ---
    println!("\n--- 2. Array Slice ---");
    let numbers = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    
    // 배열의 일부를 슬라이스로 생성
    let slice = &numbers[2..6]; // [30, 40, 50, 60]

    println!("Original Array: {:?}", numbers);
    println!("Slice (index 2 to 5): {:?}", slice);

    // 슬라이스를 함수로 전달
    print_slice(slice);

    // --- 3. 슬라이스 기반 함수 ---
    println!("\n--- 3. Slice-based Function ---");
    let part = &numbers[5..]; // index 5부터 끝까지
    print_slice(part);
}

// 슬라이스를 인자로 받는 함수 (제네릭 타입 T 사용)
fn print_slice<T: std::fmt::Debug>(slice: &[T]) {
    println!("Slice content: {:?}", slice);
    println!("Slice length: {}", slice.len());
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **범위 연산자:** `[start..end]`는 `start`를 포함하고 `end`를 포함하지 않는다는 점을 확인하세요.
2. **유니코드 주의:** 만약 문자열에 한글이 포함되어 있다면, `&s[0..3]` 처럼 바이트 경계가 맞지 않는 곳을 슬라이싱할 때 패닉(Panic)이 발생하는 것을 관찰해 보세요. (한글은 한 글자당 3바이트를 차지함)
3. **효율성:** 슬라이스는 원본 데이터를 복사하지 않고 원본의 주소와 길이만 가집니다. 따라서 매우 빠르고 메모리 효율적입니다.

## 5. 도전 과제 (Extra Credit)
- `print_slice` 함수를 수정하여 슬라이스의 합계(숫자일 경우)나 평균을 계산하는 기능을 추가해 보세요.
- 특정 값(예: `40`)이 슬라이스 내에 있는지 확인하는 기능을 만들어 보세요.
