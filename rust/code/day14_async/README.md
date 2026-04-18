# Day 14 실습 가이드: Async Programming (Tokio Basics)

이 가이드는 Rust의 비동기(Asynchronous) 프로그래밍 모델과 `Tokio` 런타임을 사용하여 효율적인 동시성 애플리케이션을 작성하는 방법을 배웁니다.

## 1. 목표
- `async/await` 문법을 사용하여 비동기 함수를 정의하고 호출한다.
- `tokio::spawn`을 사용하여 경량 비동기 Task를 생성하고 병렬로 실행한다.
- `tokio::sync::mpsc` 채널을 사용하여 비동기 환경에서 데이터를 주고받는다.
- `tokio::select!`를 사용하여 여러 비동기 작업 중 먼저 완료되는 것을 처리하거나 타임아웃을 구현한다.
- `Arc<tokio::sync::Mutex<T>>`를 사용하여 안전하게 공유 가능한 가변 데이터를 다룬다.

## 2. 프로젝트 구조
```text
day14_async/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 설정
`Cargo.toml`에 `tokio` 의존성이 포함되어 있는지 확인하세요.
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고 실험해 보세요.

```rust
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

// --- 1. Async/Await Basics ---
async fn async_hello(name: &str) {
    sleep(Duration::from_millis(500)).await;
    println!("Hello, {}! (after 500ms)", name);
}

// --- 2. Tokio Spawn & Join ---
async fn spawn_tasks() {
    println!("\n--- 2. Tokio Spawn & Join ---");
    let mut handles = vec![];

    for i in 0..5 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(i * 100)).await;
            format!("Task {} completed", i)
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("Task failed: {:?}", e),
        }
    }
}

// --- 3. MPSC in Async ---
async fn async_channel() {
    println!("\n--- 3. Async MPSC Channel ---");
    let (tx, mut rx) = mpsc::channel(32);

    // Producer task
    tokio::spawn(async move {
        for i in 1..=5 {
            let msg = format!("Message {}", i);
            if let Err(e) = tx.send(msg).await {
                eprintln!("Send error: {}", e);
                break;
            }
            sleep(Duration::from_millis(200)).await;
        }
    });

    // Consumer
    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}

// --- 4. Tokio Select (Timeout pattern) ---
async fn select_timeout() {
    println!("\n--- 4. Tokio Select (Timeout pattern) ---");
    
    let slow_task = async {
        sleep(Duration::from_secs(3)).await;
        "Slow task finished"
    };

    tokio::select! {
        res = slow_task => println!("Result: {}", res),
        _ = sleep(Duration::from_secs(1)) => println!("Error: Task timed out!"),
    }
}

// --- 5. Shared State in Async (Arc + Mutex) ---
async fn shared_state_async() {
    println!("\n--- 5. Shared State (Arc + tokio::sync::Mutex) ---");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut num = counter_clone.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    println!("Final Counter: {}", *counter.lock().await);
}

#[tokio::main]
async fn main() {
    println!("--- 1. Async/Await Basics ---");
    async_hello("Rustacean").await;

    spawn_tasks().await;
    async_channel().await;
    select_timeout().await;
    shared_state_async().await;
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **`await`의 의미:** `async_hello("Rustacean")`만 호출했을 때와 `.await`를 붙였을 때의 차이를 이해하세요.
2. **`tokio::spawn` vs `join!`:** `spawn`은 새로운 Task를 런타임에 던지는 것이고, `join!`은 현재 Task 내에서 여러 Future를 병렬로 기다리는 것입니다. 어떤 차이가 있을까요?
3. **`tokio::select!`:** 이 매크로가 왜 비동기 프로그래밍에서 "이벤트 루프"와 유사한 역할을 하는지 관찰하세요.
4. **Async Mutex:** 왜 `std::sync::Mutex` 대신 `tokio::sync::Mutex`를 사용해야 할까요? (비동기 함수 내에서 `lock()`을 호출할 때 스레드를 차단(block)하지 않기 위함입니다.)

## 5. 도전 과제 (Extra Credit)
- `tokio::task::spawn_blocking`을 사용하여 CPU 집약적인 연산(예: 큰 숫자의 소수 판별)을 비동기 Task 내에서 수행해 보세요.
- `mpsc` 채널의 버퍼 크기(capacity)를 1로 설정했을 때, Producer가 어떻게 동작하는지 확인해 보세요 (Backpressure 개념).
- `tokio::sync::broadcast`를 사용하여 여러 수신자가 동일한 메시지를 받는 구조를 만들어 보세요.
