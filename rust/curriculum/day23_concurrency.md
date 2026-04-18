# Day 23: Concurrency & Parallelism - Threads, Sync, and Send

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 강력한 동시성(Concurrency) 모델을 이해하고, 데이터 경합(Data Race) 없이 안전하게 멀티스레드 프로그래밍을 수행하는 방법을 배우는 것입니다.

* `std::thread`를 이용한 기본 스레드 생성 및 관리
* `Send`와 `Sync` 트레이트의 의미와 역할 이해
* 공유 데이터 접근을 위한 `Arc`와 `Mutex` 사용법 숙지
* 메시지 패싱(Message Passing)을 이용한 동시성 제어 (`mpsc`)
* 데이터 경합(Data Race)과 데드락(Deadlock)의 개념 및 방지 전략

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:45 | **Concurrency vs Parallelism** | 개념 차이, OS 스레드, Rust의 철학 |
| 00:45 - 01:30 | **Threads & Ownership** | `thread::spawn`, 클로저와 소유권 이전 (`move`) |
| 01:30 - 02:30 | **Send & Sync Traits** | 타입 안전성을 보장하는 마커 트레이트의 원리 |
| 02:30 - 03:30 | **Shared State (Arc & Mutex)** | 참조 카운팅과 상호 배제, `MutexGuard`의 생명주기 |
| 03:30 - 04:00 | **Message Passing (mpsc)** | 채널을 이용한 공유 메모리 없는 통신 (Do not communicate by sharing memory...) |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Send & Sync
Rust의 동시성 안전성은 이 두 가지 마커 트레이트에 기반합니다.
* **`Send`**: 타입 `T`가 다른 스레드로 안전하게 소유권을 넘길 수 있음을 의미합니다.
* **`Sync`**: 타입 `T`의 참조(`&T`)를 여러 스레드에서 동시에 안전하게 공유할 수 있음을 의미합니다. (`T: Sync` $\iff$ `&T: Send`)

### 3.2. Arc (Atomic Reference Counted)
여러 스레드가 동일한 데이터를 소유해야 할 때 사용합니다. 일반적인 `Rc`와 달리 원자적 연산을 사용하여 멀티스레드 환경에서 참조 카운트를 안전하게 관리합니다.

### 3.3. Mutex (Mutual Exclusion)
데이터에 대한 접근을 한 번에 하나의 스레드만 허용하도록 제한합니다. `Mutex`는 데이터를 보호하는 것이 아니라, 데이터를 감싸는 **Lock** 역할을 하며, `lock()` 호출 시 `MutexGuard`를 반환하여 RAII 패턴으로 락을 관리합니다.

### 3.4. Message Passing
"메모리를 공유하여 통신하지 말고, 통신을 통해 메모리를 공유하라"는 철학을 구현합니다. `std::sync::mpsc` (Multi-Producer, Single-Consumer) 채널을 사용하여 스레드 간에 데이터를 안전하게 전달합니다.

## 4. 데모 포인트 (Demonstration Points)

1. **Ownership Error:** 스레드 내부에서 외부 변수를 참조하려 할 때 발생하는 컴파일 에러와 `move` 키워드의 해결 방법.
2. **Data Race 방지:** `Arc<Mutex<T>>`를 사용하여 여러 스레드가 하나의 카운터를 안전하게 증가시키는 모습.
3. **Deadlock 상황:** 두 개의 스레드가 서로 다른 두 개의 Mutex를 교차해서 잡으려 할 때 발생하는 데드락 재현.
4. **Channel Demo:** 생산자 스레드들이 소비자 스레드에게 데이터를 보내고 처리하는 흐름.

## 5. 실습 과제 (Lab/Assignment)

### 과제: 멀티스레드 웹 크롤러 시뮬레이터 구현

**요구사항:**
1. `URL` 리스트(문자열 벡터)를 준비하세요.
2. `Arc<Mutex<Vec<String>>>`를 사용하여 "방문 완료된 URL" 목록을 관리하세요.
3. 4개의 워커 스레드를 생성하세요.
4. 각 워커 스레드는 URL 리스트에서 하나씩 URL을 가져와 (작업 큐 역할) "크롤링 중..." 메시지를 출력하고, 완료되면 "방문 완료" 목록에 추가합니다.
5. 모든 작업이 끝나면 최종적으로 방문된 URL의 개수를 출력하세요.

**검증 방법:**
- `cargo test`를 통해 모든 URL이 중복 없이 한 번씩 처리되는지 확인하세요.
- `cargo clippy`를 실행하여 동시성 관련 경고가 없는지 확인하세요.
