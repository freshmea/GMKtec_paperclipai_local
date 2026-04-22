# Day 27 실습 가이드: Advanced Traits & Lifetimes

이 가이드는 Rust의 가장 까다롭지만 강력한 기능인 Lifetime과 이를 활용한 고급 Trait 설계 패턴을 다룹니다. 참조의 유효 범위를 명시적으로 관리하고, Trait Bound에 Lifetime 제약을 결합하여 안전하고 유연한 인터페이스를 설계하는 방법을 배웁니다.

## 1. 목표
- **Explicit Lifetimes:** 구조체와 함수에서 참조의 수명을 명시적으로 선언하고 관리한다.
- **Lifetime Elision:** 컴파일러의 수명 추론 규칙을 이해하고, 추론이 불가능한 상황에서 명시적 선언을 사용한다.
- **Advanced Trait Bounds:** Trait Bound에 Lifetime 제약을 결합하여 타입 안전성을 극대화한다.

## 2. 프로젝트 구조
```text
day27_advanced_traits_lifetimes/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 의존성 확인
`Cargo.toml`에 별도의 외부 라이브러리가 필요하지 않음을 확인합니다.

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
// (코드 생략 - 위에서 작성한 내용과 동일)
```

## 4. 실행 및 테스트
```bash
cargo run
cargo test
```

**실습 포인트:**
1. **Lifetime Annotations:** `Book<'a>`에서 `'a`가 무엇을 의미하는지, 만약 이 annotation이 없다면 왜 컴파일 에러가 발생하는지 분석해 보세요.
2. **Function Lifetimes:** `longest<'a>(x: &'a str, y: &'a str) -> &'a str` 함수에서 반환값의 수명이 입력값 중 하나와 연결되는 이유를 이해해 보세요.
3. **Trait + Lifetime:** `Validator<'a>` Trait을 구현할 때, `validate` 메서드의 입력 파라미터 수명과 Trait 자체의 수명이 어떻게 상호작용하는지 관찰해 보세요.

## 5. 도전 과제 (Extra Credit)
- **Lifetime Subtyping:** 서로 다른 수명을 가진 참조들 사이의 관계(Subtyping)를 조사하고, `&'a str`이 `&'b str`로 전달될 수 있는 조건을 실험해 보세요.
- **Self Lifetime:** 메서드 정의 시 `&self`의 수명을 명시적으로 사용하는 경우(`fn method<'a>(&'a self) -> ...`) 어떤 차이가 있는지 조사해 보세요.
- **Complex Lifetimes:** 구조체가 두 개 이상의 참조를 가지고 있고, 그 참조들의 수명이 서로 다를 때 어떻게 정의해야 하는지 코드로 구현해 보세요.
