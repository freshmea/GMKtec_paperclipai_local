# Day 14: Async Programming (Tokio Basics)

## 1. 학습 목표 (Objectives)
- Rust의 비동기(Asynchronous) 프로그래밍 모델과 `Future` 트레이트의 개념을 이해합니다.
- `async/await` 문법을 사용하여 비동기 코드를 작성하는 방법을 배웁니다.
- Rust의 표준 비동기 런타임인 `Tokio`의 핵심 컴포넌트(Runtime, Task, Spawning)를 익힙니다.
- 비동기 환경에서의 블로킹(Blocking) 문제와 이를 해결하기 위한 `spawn_blocking`의 용도를 파악합니다.

## 2. 세션 일정 (Session Breakdown)

| 시간 (Time) | 주제 (Topic) | 내용 (Details) |
| :--- | :--- | :--- |
| 00:00 - 00:45 | 비동기 프로그래밍 기초 | Sync vs Async, Future 트레이트, Polling 모델 |
| 00:45 - 01:30 | async/await 문법 | async 함수 정의, await를 통한 결과 대기, 제어 흐름 |
| 01:30 - 02:30 | Tokio Runtime 기초 | `#[tokio::main]`, `tokio::spawn`, Task의 개념 |
| 02:30 - 03:30 | 비동기 I/O 및 병렬 실행 | `tokio::join!`, `tokio::select!`, 비동기 타이머 |
| 03:30 - 04:00 | 종합 실습 및 Q&A | 실습 코드 실행 및 비동기 설계 시 주의사항 (Blocking 방지) |

## 3. 핵심 개념 (Key Concepts)

### 3.1 비동기란 무엇인가? (Async vs Sync)
- **Sync (동기):** 작업이 완료될 때까지 스레드가 대기(Block)합니다. CPU가 아무것도 하지 않고 기다리는 시간이 발생합니다.
- **Async (비동기):** 작업이 완료될 때까지 기다리는 동안, 스레드는 다른 작업을 수행할 수 있습니다. 이를 통해 적은 수의 스레드로 수만 개의 동시 연결을 처리할 수 있습니다.

### 3.2 Future 트레이트
Rust에서 비동기 작업의 결과는 `Future`라는 트레이트로 추상화됩니다.
- `Future`는 "나중에 완료될 작업"을 나타내는 상태 머신입니다.
- `poll` 메서드를 호출하여 작업이 완료되었는지(`Ready`) 아니면 아직 진행 중인지(`Pending`)를 확인합니다.
- **주의:** Rust의 Future는 `await`를 하거나 런타임에 의해 `poll`되지 않으면 **아무 일도 일어나지 않습니다.**

### 3.3 async/await 문법
- **`async fn`:** 함수를 호출했을 때 즉시 실행되는 대신, `Future`를 반환하도록 만듭니다.
- **`.await`:** Future가 완료될 때까지 현재 실행 흐름을 일시 중단(suspend)하고, 완료되면 결과를 반환받습니다.

### 3.4 Tokio Runtime
Rust 표준 라이브러리는 비동기 실행 엔진(Executor)을 포함하지 않습니다. 따라서 `Tokio`와 같은 런타임이 필요합니다.
- **Runtime:** 비동기 작업(Task)을 스케줄링하고 실행하는 엔진입니다.
- **`tokio::spawn`:** 새로운 비동기 Task를 생성하여 런타임에 넘깁니다. 이는 OS 스레드 생성보다 훨씬 가볍습니다.
- **`tokio::select!`:** 여러 비동기 작업을 동시에 실행하고, 가장 먼저 완료되는 작업의 결과를 처리합니다.

### 3.5 비동기 환경에서의 주의사항: Blocking
비동기 스레드(Worker thread) 내에서 `std::thread::sleep`이나 무거운 연산(CPU-bound)을 수행하면, 해당 스레드가 점유되어 다른 비동기 Task들이 실행되지 못하는 **Starvation** 현상이 발생합니다.
- **해결책:** 
  - 비동기용 타이머 사용 (`tokio::time::sleep`)
  - 무거운 연산은 `tokio::task::spawn_blocking`을 사용하여 별도의 전용 스레드 풀로 넘깁니다.

## 4. 데모 포인트 (Demonstration Points)
1. `async/await`를 사용한 간단한 네트워크/타이머 시뮬레이션.
2. `tokio::spawn`을 이용한 여러 Task의 병렬 실행.
3. `tokio::join!`을 사용하여 여러 Future를 동시에 기다리기.
4. `tokio::select!`를 사용하여 타임아웃 처리 구현하기.

## 5. 실습 과제 (Lab/Assignment)

### 과제 1: 비동기 타이머 (Async Timer)
`tokio::time::sleep`을 사용하여 지정된 시간마다 메시지를 출력하는 비동기 루프를 구현하세요.

### 과제 2: 병렬 작업 처리 (Task Spawning)
10개의 비동기 작업을 `tokio::spawn`으로 생성하고, 각 작업이 완료될 때마다 결과를 수집하는 프로그램을 작성하세요.

### 과제 3: 타임아웃 구현 (Select/Timeout)
비동기 작업이 특정 시간(예: 2초) 내에 완료되지 않으면 "Timeout!" 메시지를 출력하고 중단하는 `tokio::select!` 기반의 프로그램을 작성하세요.
