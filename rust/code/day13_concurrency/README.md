# Day 13 실습 가이드: Concurrency (Threads & Channels)

이 가이드는 Rust의 강력한 동시성 모델을 사용하여 여러 작업을 병렬로 처리하고, 스레드 간에 안전하게 데이터를 주고받는 방법을 배웁니다.

## 1. 목표
- `std::thread::spawn`을 사용하여 새로운 스레드를 생성하고 관리한다.
- `move` 키워드를 사용하여 클로저가 스레드 내부로 소유권을 전달하는 방식을 이해한다.
- `mpsc` 채널을 사용하여 Producer-Consumer 패턴을 구현한다.
- `Arc<Mutex<T>>` 조합을 사용하여 여러 스레드에서 안전하게 공유 가능한 가변 데이터를 다룬다.

## 2. 프로젝트 구조
```text
day13_concurrency/
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
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

// --- 1. Threads: Parallel Computation ---
fn parallel_squares(numbers: Vec<i32>) -> Vec<i32> {
    let mut handles = vec![];

    for num in numbers {
        // 'move'를 사용하여 num의 소유권을 스레드 내부로 가져옵니다.
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            num * num
        });
        handles.push(handle);
    }

    // 모든 스레드가 완료될 때까지 기다린 후 결과를 수집합니다.
    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}

// --- 2. Channels: Message Passing Pipeline ---
fn channel_pipeline() -> i32 {
    // mpsc: Multi-Producer, Single-Consumer
    let (tx, rx) = mpsc::channel();

    // 3개의 Producer 스레드 생성
    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let val = i * 10;
            println!("Producer {}: sending {}", i, val);
            tx_clone.send(val).unwrap();
        });
    }

    // 원본 송신자를 드롭해야 rx가 모든 메시지를 받은 후 루프를 종료할 수 있습니다.
    drop(tx);

    let mut sum = 0;
    for received in rx {
        println!("Received: {}", received);
        sum += received;
    }
    sum
}

// --- 3. Arc & Mutex: Shared State ---
fn shared_counter() -> i32 {
    // Arc: Atomic Reference Counting (여러 스레드 간 소유권 공유)
    // Mutex: Mutual Exclusion (데이터 가변성 보장)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // lock()을 통해 데이터에 대한 배타적 접근 권한을 얻습니다.
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *counter.lock().unwrap();
    result
}

fn main() {
    println!("--- 1. Threads: Parallel Squares ---");
    let nums = vec![1, 2, 3, 4, 5];
    let squares = parallel_squares(nums);
    println!("Squares: {:?}", squares);

    println!("\n--- 2. Channels: Message Passing ---");
    let total_sum = channel_pipeline();
    println!("Total Sum from Channels: {}", total_sum);

    println!("\n--- 3. Arc & Mutx: Shared Counter ---");
    let final_count = shared_counter();
    println!("Final Shared Counter: {}", final_count);
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **`move` 키워드:** `parallel_squares`에서 `move`를 제거했을 때 발생하는 컴파일 에러를 확인하세요. 스레드가 외부 변수의 수명(lifetime)을 보장할 수 없기 때문입니다.
2. **채널 종료:** `drop(tx)`가 왜 필요한지 생각해보세요. 만약 원본 송신자가 살아있다면 `rx` 루프는 언제 끝날까요?
3. **Mutex Lock:** `counter_clone.lock().unwrap()` 호출 시, 만약 다른 스레드가 이미 lock을 쥐고 있다면 현재 스레드는 어떻게 동작하나요?

## 5. 도전 과제 (Extra Credit)
- `mpsc` 대신 `sync_channel`을 사용하여 버퍼 크기가 제한된 채널을 만들어 보세요.
- `Arc<Mutex<T>>` 대신 `Arc<RwLock<T>>`를 사용하여 읽기 작업이 많은 경우의 성능 차이를 고민해 보세요.
- 여러 개의 Producer가 데이터를 보낼 때, 특정 조건(예: 합계가 100이 되면)에서 채널을 닫는 로직을 추가해 보세요.
