# Day 1. Rust 소개와 개발 환경 (상세 실습 가이드)

이 문서는 Day 1 강의의 실습을 돕기 위한 가이드입니다.

## 1. 개발 환경 준비

먼저 Rust가 제대로 설치되었는지 확인합니다.

```bash
rustup --version
cargo --version
```

## 2. Hello Rust!

첫 번째 Cargo 프로젝트를 생성합니다.

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

`src/main.rs`를 열어 코드를 확인해보세요.

## 3. 변수와 불변성 (Immutability)

`hello_rust/src/main.rs` 파일을 다음과 같이 수정하여 불변성과 가변성을 테스트합니다.

```rust
fn main() {
    // 1. 불변 변수 (기본값)
    let x = 5;
    println!("x is: {}", x);

    // 아래 주석을 해제하면 컴파일 에러가 발생합니다!
    // x = 6; 

    // 2. 가변 변수
    let mut y = 10;
    println!("y is: {}", y);
    y = 15;
    println!("y is now: {}", y);
}
```

## 4. 섀도잉 (Shadowing)

변수의 타입을 변경하고 싶을 때 섀도잉을 사용합니다.

```rust
fn main() {
    let spaces = "   "; // 문자열 타입
    let spaces = spaces.len(); // 숫자 타입으로 섀도잉

    println!("Number of spaces: {}", spaces);
}
```

## 5. 튜플과 배열

```rust
fn main() {
    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // 패턴 매칭 분해
    println!("a: {}, b: {}, c: {}", a, b, c);

    // 배열
    let arr = [1, 2, 3, 4, 5];
    println!("First element: {}", arr[0]);
}
```
