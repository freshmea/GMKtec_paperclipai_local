# Day 24 실습 가이드: Async/Await with Tokio

이 가이드는 Rust의 비동기 프로그래밍 모델을 이해하고, 가장 널리 쓰이는 런타임인 `Tokio`를 사용하여 효율적인 I/O 중심 애플리케이션을 작성하는 방법을 배우는 것을 목표로 합니다.

## 1. 목표
- `async/await` 문법을 사용하여 비동기 흐름을 제어한다.
- `tokio::join!`을 사용하여 여러 Future를 동시에 실행한다.
- `tokio::spawn`을 사용하여 독립적인 비동기 태스크를 생성한다.
- 비동기 작업의 순차 실행과 병렬 실행의 성능 차이를 이해한다.

## 2. 프로젝트 구조
```text
day24_async/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 의존성 확인
`Cargo.toml`에 `tokio` (full features)와 `futures`가 포함되어 있는지 확인합니다.

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
use tokio;

async fn fetch_data(id: i32) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    format!("Data from source {}", id)
}

async fn run_sequential() {
    let start = std::time::Instant::now();
    let res1 = fetch_data(1).await;
    let res2 = fetch_data(2).await;
    println!("Sequential: {:?}, {:?} (took {:?})", res1, res2, start.elapsed());
}

async fn run_concurrent() {
    let start = std::time::Instant::now();
    let (res1, res2) = tokio::join!(fetch_data(1), fetch_data(2));
    println!("Concurrent: {:?}, {:?} (took {:?})", res1, res2, start.elapsed());
}

#[tokio::main]
async fn main() {
    run_sequential().await;
    run_concurrent().await;
}
```

## 4. 실행 및 테스트
```bash
cargo run
cargo test
```

**실습 포인트:**
1. **`await`의 의미:** `await`를 호출할 때 현재 실행 중인 함수가 어떻게 일시 중단(suspend)되고 제어권이 런타임으로 넘어가는지 이해해 보세요.
2. **`join!` vs `spawn`:** `tokio::join!`은 현재 태스크 내에서 여러 Future를 기다리는 것이고, `tokio::spawn`은 새로운 태스크를 런타임 스케줄러에 할당하는 것입니다. 두 방식의 차이점을 고민해 보세요.
3. **Cancellation:** `tokio::select!`를 사용하여 여러 비동기 작업 중 하나가 완료되면 나머지를 취소하는 패턴을 조사해 보세요.

## 5. 도전 과제 (Extra Credit)
- `tokio::select!`를 사용하여 타임아웃(timeout) 로직을 구현해 보세요. (예: `fetch_data`가 300ms 안에 안 오면 에러 반환)
- `tokio::sync::mpsc` 채널을 사용하여 Producer-Consumer 패턴을 비동기적으로 구현해 보세요.
- 여러 개의 `tokio::spawn` 태스크를 만들고, `JoinHandle`을 사용하여 모든 결과가 올 때까지 기다리는 코드를 작성해 보세요.
