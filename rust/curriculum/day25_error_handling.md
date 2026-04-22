# Day 25: Advanced Error Handling - Thiserror, Anyhow, and Custom Error Types

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 에러 처리 철학을 깊이 있게 이해하고, 라이브러리와 애플리케이션 개발 시 각각에 최적화된 에러 처리 전략을 수립하는 방법을 배우는 것입니다.

* Rust의 `Result<T, E>`와 `Option<T>`를 이용한 에러 처리의 근본 원리 이해
* `thiserror`를 이용한 라이브러리용 커스텀 에러 타입 설계
* `anyhow`를 이용한 애플리케이션용 유연한 에러 처리 및 Context 추가
* 에러 전파(`?` 연산자)와 에러 변환(`From` 트레이트)의 숙달
* 에러 컨텍스트(Error Context)를 통한 디버깅 효율성 극대화

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:45 | **Error Handling Philosophy** | Panic vs Result, 에러의 종류 (Recoverable vs Unrecoverable) |
| 00:45 - 01:45 | **Library Error Design** | `thiserror`를 활용한 명확하고 구체적인 에러 타입 정의 |
| 01:45 - 02:45 | **Application Error Handling** | `anyhow`를 활용한 에러 전파 및 `context()`를 통한 정보 추가 |
| 02:45 - 03:30 | **Error Conversion** | `From` 트레이트를 이용한 에러 타입 간의 자동 변환 및 `?` 활용 |
| 03:30 - 04:00 | **Lab & Assignment** | 실습: 라이브러리 에러를 애플리케이션 에러로 변환하는 계층 구조 구현 |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Library vs Application Error
* **Library (thiserror):** 라이브러리는 사용자가 에러를 프로그래밍적으로 처리할 수 있어야 합니다. 따라서 에러 타입이 **구체적(Specific)**이고 **예측 가능**해야 합니다.
* **Application (anyhow):** 애플리케이션은 에러를 발생시키기보다 사용자에게 정보를 전달하고 로그를 남기는 것이 주 목적입니다. 따라서 에러 타입이 **유연(Flexible)**하고 **풍부한 맥락(Context)**을 가져야 합니다.

### 3.2. `thiserror` (For Libraries)
`thiserror`는 커스텀 에러 타입을 정의할 때 발생하는 반복적인 보일러플레이트(Display, Error 트레이트 구현 등)를 매크로로 해결해 줍니다.
```rust
#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid user ID: {0}")]
    InvalidId(u32),
}
```

### 3.3. `anyhow` (For Applications)
`anyhow`는 다양한 종류의 에러를 하나의 `anyhow::Error`로 묶어서 처리할 수 있게 해줍니다. 특히 `.context()` 메서드를 통해 에러가 발생한 위치나 상황에 대한 정보를 추가할 수 있어 디버깅에 매우 강력합니다.
```rust
use anyhow::{Context, Result};

fn run_app() -> Result<()> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to load configuration file")?; // 에러에 맥락 추가
    Ok(())
}
```

### 3.4. The `?` Operator
`?` 연산자는 에러가 발생하면 즉시 함수를 반환하고, 에러가 발생하지 않으면 값을 반환합니다. 이때 `From` 트레이트가 구현되어 있다면 에러 타입을 자동으로 변환해 줍니다.

## 4. 데모 포인트 (Demonstration Points)

1. **Manual Error Implementation:** `std::fmt::Display`와 `std::error::Error`를 직접 구현하며 느끼는 번거로움 보여주기.
2. **thiserror Magic:** 위 번거로움을 `thiserror`가 얼마나 깔끔하게 해결하는지 비교.
3. **Contextual Debugging:** `anyhow`의 `.context()`를 사용하여 에러가 발생했을 때 단순한 "Permission Denied"가 아니라 "Failed to open user database: Permission Denied"와 같이 친절한 로그가 찍히는 모습 확인.

## 5. 실습 과제 (Lab/Assignment)

### 과제: 계층형 에러 처리 시스템 구축

**요구사항:**
1. **`database_lib` (Library 역할):**
   - `DbError` enum을 `thiserror`로 정의하세요. (`ConnectionError`, `QueryError`, `NotFound`)
   - 데이터를 조회하는 `fetch_user(id: u32)` 함수를 만드세요. (실제 DB 대신 에러를 임의로 발생시킴)
2. **`app_service` (Application 역할):**
   - `anyhow`를 사용하여 `run_service()` 함수를 만드세요.
   - `database_lib`의 함수를 호출하고, 에러 발생 시 `.context("Failed to fetch user from database")`를 추가하세요.
3. **Main 함수:**
   - `run_service()`를 호출하고, 에러가 발생하면 `{:?}`(Debug) 형식을 사용하여 에러의 전체 체인(Root cause + Context)을 출력하세요.

**검증 방법:**
- `cargo test`를 통해 에러가 발생했을 때 의도한 컨텍스트가 포함되어 출력되는지 확인하세요.
- `cargo clippy`를 통해 에러 처리 로직의 품질을 확인하세요.
