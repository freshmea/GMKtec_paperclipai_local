# Day 7 실습 가이드: 참조의 수명 기초 (Lifetimes Basics)

이 가이드는 Rust 컴파일러가 참조의 유효성을 어떻게 검사하는지, 그리고 왜 때때로 개발자가 직접 'Lifetime Annotation'을 작성해야 하는지 배우는 것을 목표로 합니다.

## 1. 목표
- Dangling Reference(허공을 가리키는 참조)가 무엇인지 이해한다.
- 컴파일러가 Lifetime을 추론하는 원리를 파악한다.
- 함수 시그니처에 Lifetime Annotation (`'a`)을 사용하여 참조 간의 관계를 명시한다.

## 2. 프로젝트 구조
```text
day7_lifetime_basics/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day7_lifetime_basics
cd day7_lifetime_basics
```

### 단계 2: Lifetime 에러와 해결 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 주석 처리된 부분을 하나씩 해제하며 컴파일러와 대화해 보세요.

```rust
// --- 1. Dangling Reference (에러 유도) ---
// 이 함수는 스코프가 끝나면 사라지는 지역 변수의 참조를 반환하려고 시도합니다.
/*
fn dangle() -> &String {
    let s = String::from("I am local");
    &s // 에러! s는 함수가 끝나면 드롭되므로, 이 참조는 허공을 가리키게 됩니다.
}
*/

// --- 2. Lifetime Annotation을 이용한 해결 ---
// 두 개의 문자열 슬라이스 중 더 긴 것을 반환하는 함수입니다.
// 'a는 입력받은 두 참조가 모두 최소한 'a만큼의 수명을 가져야 함을 의미합니다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // --- 실습 A: 정상 작동 ---
    let string1 = String::from("long string");
    let string2 = "short";

    let result = longest(string1.as_str(), string2);
    println!("가장 긴 문자열은: {}", result);

    // --- 실습 B: Lifetime 문제 발생 상황 ---
    let string3 = String::from("hello");
    let result2;
    
    {
        let string4 = String::from("world");
        result2 = longest(string3.as_str(), string4.as_str());
        println!("Scope 안의 결과: {}", result2);
    } // string4는 여기서 드롭됩니다.

    // println!("Scope 밖의 결과: {}", result2); 
    // [실습] 위 주석을 해제해보세요. 
    // result2는 string4의 수명에 묶여 있기 때문에, string4가 사라진 후에는 사용할 수 없습니다.
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 가이드:**
1. `dangle` 함수의 주석을 풀고 발생하는 에러 메시지를 읽어보세요. "returns a reference to data owned by the current function"이라는 메시지가 핵심입니다.
2. `longest` 함수에서 `'a`를 제거했을 때 컴파일 에러가 발생하는지 확인하세요. 컴파일러는 반환되는 참조가 입력된 참조 중 어느 것에 수명을 맞춰야 할지 알 수 없기 때문입니다.
3. `result2` 사용 시 발생하는 에러를 통해, Lifetime이 단순히 "길이"가 아니라 "참조 간의 관계"임을 이해합니다.

## 5. 도전 과제 (Extra Credit)
- `longest` 함수가 `&str`이 아닌 `&String`을 인자로 받도록 수정해 보세요. (힌트: `&String`은 `&str`로 자동 형변환(Deref coercion)이 가능합니다.)
- `longest` 함수를 호출할 때, `string1`과 `string4` 중 어느 쪽의 수명이 `result2`의 수명을 결정하는지 관찰해 보세요.
