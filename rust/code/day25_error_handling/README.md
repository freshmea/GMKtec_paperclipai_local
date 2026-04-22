# Day 25 실습 가이드: Error Handling (thiserror & anyhow)

이 가이드는 Rust에서 가장 권장되는 에러 처리 패턴인 `thiserror`와 `anyhow`를 사용하여, 라이브러리와 애플리케이션 레이어에서 각각 어떻게 에러를 정의하고 관리하는지 배우는 것을 목표로 합니다.

## 1. 목표
- `thiserror`를 사용하여 라이브러리용 구조화된 커스텀 에러 타입을 정의한다.
- `anyhow`를 사용하여 애플리케이션 레이어에서 에러에 컨텍스트를 추가하고 편리하게 전파한다.
- 에러 체이닝(Error Chaining)을 통해 근본 원인(Root Cause)을 추적하는 법을 익힌다.

## 2. 프로젝트 구조
```text
day25_error_handling/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 의존성 확인
`Cargo.toml`에 `thiserror`와 `anyhow`가 포함되어 있는지 확인합니다.

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
1. **라이브러리 vs 애플리케이션:** 왜 라이브러리에서는 `thiserror`를 쓰고, 애플리케이션에서는 `anyhow`를 쓰는지 그 차이점을 고민해 보세요.
2. **`context()`의 힘:** `anyhow`의 `.context()` 또는 `.with_context()`가 에러 디버깅 시 얼마나 큰 도움을 주는지 확인해 보세요.
3. **`?` 연산자와 에러 변환:** `DbError`가 어떻게 `anyhow::Error`로 자동으로 변환되는지 이해해 보세요.

## 5. 도전 과제 (Extra Credit)
- `thiserror`를 사용하여 `PermissionDenied` 에러를 추가하고, `fetch_user` 함수에서 특정 ID(예: 13)에 대해 이 에러를 반환하도록 수정해 보세요.
- `anyhow`의 `bail!` 매크로를 사용하여 특정 조건에서 즉시 에러를 반환하는 로직을 추가해 보세요.
- 에러 발생 시 `tracing` 라이브러리를 사용하여 에러 로그를 구조화된 형태로 남기는 법을 조사해 보세요.
