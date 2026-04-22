# Day 24: Async Rust - Fundamentals, Tokio, and Runtime

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 비동기(Asynchronous) 프로그래밍 모델을 이해하고, 가장 널리 쓰이는 런타임인 `Tokio`를 사용하여 고성능 비동기 서비스를 구축하는 방법을 배우는 것입니다.

* `Future` 트레이트의 개념과 작동 원리 이해
* `.await`의 의미와 비동기 함수의 실행 흐름 파악
* `async/await` 문법을 이용한 비동기 코드 작성
* `Tokio` 런타임의 구조와 스레드 모델 이해
* 비동기 I/O, 타이머, 그리고 비동기 작업의 병렬 실행 (`tokio::spawn`)

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:45 | **Async Fundamentals** | Blocking vs Non-blocking, State Machine으로서의 Future |
| 00:45 - 01:30 | **Async/Await Syntax** | `async fn`, `.await`, Future의 지연 실행(Lazy) 특성 |
| 01:30 - 02:30 | **Tokio Runtime** | Multi-threaded scheduler, Worker threads, Task spawning |
| 02:30 - 03:30 | **Async I/O & Timers** | `tokio::net`, `tokio::time`, 비동기 에코 서버 기초 |
| 03:30 - 04:00 | **Lab & Assignment** | 실습: 비동기 HTTP 요청을 병렬로 수행하는 클라이언트 구현 |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Future와 Polling
Rust의 비동기는 **Pull-based** 모델입니다. `Future`는 값이 준비될 때까지 기다리는 상태를 나타내며, 런타임(Executor)이 `poll()` 메서드를 호출하여 진행 상태를 확인합니다. `Pending`이 반환되면 작업이 완료될 때까지 대기합니다.

### 3.2. Async/Await
* `async` 키워드는 함수를 호출했을 때 즉시 실행하는 대신, 나중에 실행할 수 있는 `Future` 객체를 반환하도록 만듭니다.
* `.await`는 현재 실행 흐름을 일시 중단하고, 해당 `Future`가 완료될 때까지 런타임에 제어권을 넘깁니다.

### 3.3. Tokio Runtime
Rust 표준 라이브러리에는 비동기 실행기(Executor)가 포함되어 있지 않습니다. `Tokio`는 비동기 작업을 스케줄링하고, I/O 이벤트를 처리하며, 수만 개의 가벼운 작업(Task)을 효율적으로 관리하는 엔진 역할을 합니다.

### 3.4. Task Spawning (`tokio::spawn`)
`async` 함수를 호출하는 것만으로는 병렬 실행이 되지 않습니다. `tokio::spawn`을 사용해야 새로운 독립적인 Task로 생성되어 런타임의 스케줄러에 의해 여러 스레드에서 병렬로 실행될 수 있습니다.

## 4. 데모 포인트 (Demonstration Points)

1. **Blocking vs Async:** `std::thread::sleep`을 사용하여 비동기 루프를 멈추는 문제와 `tokio::time::sleep`을 사용하는 차이 보여주기.
2. **Lazy Execution:** `async` 함수를 호출만 하고 `.await`를 하지 않았을 때 아무 일도 일어나지 않는 모습 확인.
3. **Parallelism with `spawn`:** `tokio::spawn`을 사용했을 때와 사용하지 않았을 때 전체 작업 완료 시간 비교.

## 5. 실습 과제 (Lab/Assignment)

### 과제: 비동기 병렬 HTTP 요청 클라이언트 구현

**요구사항:**
1. `reqwest`와 `tokio` 크레이트를 사용하세요.
2. 여러 개의 URL 리스트를 입력으로 받습니다.
3. `tokio::spawn`을 사용하여 각 URL에 대한 HTTP GET 요청을 **병렬로** 수행하세요.
4. 모든 요청이 완료될 때까지 기다린 후, 각 요청의 상태 코드(Status Code)를 출력하세요.
5. **핵심:** 요청이 너무 많을 경우 시스템 자원을 보호하기 위해 `Semaphore`를 사용하여 동시 요청 수를 제한(Concurrency Limit)하는 기능을 추가하세요.

**검증 방법:**
- `cargo run`을 통해 병렬로 요청이 날아가는 것을 확인하세요.
- `cargo test`를 통해 요청 성공/실패 시나리오를 테스트하세요.
