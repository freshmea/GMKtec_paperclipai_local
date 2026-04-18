# Day 13: Concurrency (Threads & Channels)

## 1. 학습 목표 (Objectives)
- Rust의 멀티스레딩 모델을 이해하고, OS 스레드를 생성하고 관리하는 법을 배웁니다.
- `Send`와 `Sync` 트레이트를 통해 스레드 간 데이터 안전성을 보장하는 원리를 파악합니다.
- `mpsc` (Multi-Producer, Single-Consumer) 채널을 사용하여 스레드 간에 데이터를 안전하게 통신하는 법을 익힙니다.
- 공유 데이터 접근 시 발생할 수 있는 Race Condition과 이를 방지하기 위한 동기화 도구의 기초를 이해합니다.

## 2. 세션 일정 (Session Breakdown)

| 시간 (Time) | 주제 (Topic) | 내용 (Details) |
| :--- | :--- | :--- |
| 00:00 - 00:45 | 스레드 기초 (Threads) | `std::thread::spawn`, 스레드 조인 (`join`), 클로저와 소유권 이동 (`move`) |
| 00:45 - 01:30 | 스레드 안전성 (Send & Sync) | `Send`와 `Sync` 트레이트의 의미, 데이터 레이스 방지 원리 |
| 01:30 - 02:30 | 메시지 패싱 (Channels) | `mpsc` 채널 사용법, Producer/Consumer 패턴, 채널을 통한 데이터 전송 |
| 02:30 - 03:30 | 공유 상태 (Shared State) | `Arc<Mutex<T>>`를 이용한 안전한 데이터 공유 및 동기화 |
| 03:30 - 04:00 | 종합 실습 및 Q&A | 실습 코드 실행 및 멀티스레드 프로그래밍 주의사항 정리 |

## 3. 핵심 개념 (Key Concepts)

### 3.1 스레드 (Threads)
Rust는 OS 스레드를 사용하여 병렬 처리를 수행합니다.
- **`thread::spawn`**: 새로운 스레드를 생성합니다.
- **`move` 키워드**: 클로저가 외부 환경의 변수를 소유하도록 강제하여, 스레드가 종료될 때까지 데이터의 생명주기를 보장합니다.
- **`join()`**: 생성된 스레드가 작업을 마칠 때까지 메인 스레드가 기다리도록 합니다.

### 3.2 Send와 Sync (Thread Safety)
Rust의 타입 시스템은 스레드 안전성을 컴파일 타임에 검증합니다.
- **`Send`**: 해당 타입을 다른 스레드로 소유권을 넘길 수 있음을 의미합니다.
- **`Sync`**: 여러 스레드에서 해당 타입의 참조(`&T`)를 동시에 공유할 수 있음을 의미합니다.

### 3.3 채널 (Channels)
"Do not communicate by sharing memory; instead, share memory by communicating."
- **`mpsc` (Multi-Producer, Single-Consumer)**: 여러 송신자가 하나의 수신자에게 메시지를 보낼 수 있는 통신 통로입니다.
- **메시지 패싱**: 데이터를 복사하거나 소유권을 넘김으로써, 공유 메모리 없이도 스레드 간 협업이 가능합니다.

### 3.4 공유 상태와 동기화 (Arc & Mutex)
데이터를 여러 스레드에서 공유하면서 동시에 수정해야 할 경우:
- **`Arc<T>` (Atomic Reference Counted)**: 스레드 간에 안전하게 소유권을 공유할 수 있게 해주는 참조 카운팅 스마트 포인터입니다.
- **`Mutex<T>` (Mutual Exclusion)**: 한 번에 하나의 스레드만 데이터에 접근할 수 있도록 잠금(Lock)을 제공합니다.
- **`Arc<Mutex<T>>`**: 이 조합은 멀티스레드 환경에서 공유 가능한 가변 데이터의 표준 패턴입니다.

## 4. 데모 포인트 (Demonstration Points)
1. `thread::spawn`과 `move`를 사용하여 병렬 작업 수행하기.
2. `mpsc` 채널을 사용하여 워커 스레드로부터 메인 스레드로 작업 결과 수집하기.
3. `Arc<Mutex<T>>`를 사용하여 여러 스레드에서 하나의 카운터를 안전하게 증가시키기.

## 5. 실습 과제 (Lab/Assignment)

### 과제 1: 병렬 계산기 (Threads)
숫자 배열을 받아 각 요소를 제곱하는 작업을 여러 스레드에 나누어 할당하고, 모든 작업이 완료될 때까지 기다린 후 결과를 출력하는 프로그램을 작성하세요.

### 과제 2: 메시지 처리 파이프라인 (Channels)
생성자(Producer) 스레드 3개가 각각 무작위 숫자를 생성하여 채널로 보내고, 수신자(Consumer) 스레드 1개가 이 숫자들을 모두 받아 합계를 계산하는 프로그램을 작성하세요.

### 과제 3: 공유 자원 관리 (Arc & Mutex)
여러 개의 스레드가 하나의 `Vec<String>`에 로그 메시지를 기록하는 시스템을 구현하세요. `Arc<Mutex<Vec<String>>>`를 사용하여 데이터 레이스 없이 안전하게 로그를 쌓아야 합니다.
