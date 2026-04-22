# Day 22 실습 가이드: Macros (Declarative & Procedural)

이 가이드는 Rust의 강력한 코드 생성 도구인 매크로를 사용하여 코드 중복을 줄이고, 새로운 문법 패턴(DSL)을 만드는 방법을 배우는 것을 목표로 합니다.

## 1. 목표
- `macro_rules!`를 사용하여 선언적 매크로(Declarative Macros)를 작성한다.
- 매크로의 패턴 매칭(`$x:expr` 등)과 반복(`$(...)*`) 문법을 이해한다.
- 절차적 매크로(Procedural Macros)의 개념과 종류(Derive, Attribute, Function-like)를 파악한다.

## 2. 프로젝트 구조
```text
day22_macros/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
(이미 생성되어 있습니다.)

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
// --- 1. Declarative Macros (macro_rules!) ---
macro_rules! vec_and_print {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("Added: {}", $x);
            )*
            temp_vec
        }
    };
}

macro_rules! calculate {
    (add $a:expr, $b:expr) => { $a + $b };
    (sub $a:expr, $b:expr) => { $a - $b };
    (mul $a:expr, $b:expr) => { $a * $b };
    (div $a:expr, $b:expr) => { $a / $b };
}

fn main() {
    println!("--- 1. Declarative Macros (macro_rules!) ---");
    
    // vec_and_print! 매크로 사용
    let my_vec = vec_and_print![10, 20, 30, 40];
    println!("Final vector: {:?}", my_vec);

    println!("\n--- 2. Simple Arithmetic Macro ---");
    let sum = calculate!(add 10, 5);
    let diff = calculate!(sub 10, 5);
    let prod = calculate!(mul 10, 5);
    let quot = calculate!(div 10, 5);

    println!("10 + 5 = {}", sum);
    println!("10 - 5 = {}", diff);
    println!("10 * 5 = {}", prod);
    println!("10 / 5 = {}", quot);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_and_print() {
        let v = vec_and_print![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate!(add 10, 5), 15);
        assert_eq!(calculate!(sub 10, 5), 5);
        assert_eq!(calculate!(mul 10, 5), 50);
        assert_eq!(calculate!(div 10, 5), 2);
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
cargo test
```

**실습 포인트:**
1. **패턴 매칭:** `$( $x:expr ),*`에서 `$`의 의미와 `$x:expr`이 무엇을 의미하는지 이해해 보세요.
2. **코드 확장:** 매크로가 컴파일 타임에 어떻게 실제 코드로 변환되는지 상상해 보세요. (매크로를 사용하면 컴파일 시간이 길어질 수 있습니다.)
3. **절차적 매크로:** `#[derive(Debug)]`가 어떻게 동작하는지, 그리고 왜 `macro_rules!`보다 강력한지 조사해 보세요.

## 5. 도전 과제 (Extra Credit)
- `vec_and_print!` 매크로를 확장하여, 인자로 전달된 요소의 개수를 함께 출력하도록 만들어 보세요.
- `calculate!` 매크로에 `pow` (거듭제곱) 연산을 추가해 보세요.
- `macro_rules!`를 사용하여 간단한 `if-else` 문을 대체하는 `if_let_macro!`를 만들어 보세요.
