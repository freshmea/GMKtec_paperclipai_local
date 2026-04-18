# Day 15: Axum을 이용한 RESTful API 개발 (Axum & RESTful API Development)

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 현대적인 웹 프레임워크인 **Axum**을 사용하여 안정적이고 확장 가능한 **RESTful API**를 구축하는 방법을 배우는 것입니다.

* **Axum 프레임워크 이해:** Axum의 핵심 설계 철학(Tokio 기반, Tower 서비스 활용)을 파악합니다.
* **Routing & Extractors:** URL 경로와 HTTP 메서드에 따라 요청을 처리하고, 요청 데이터(Path, Query, JSON, State)를 추출하는 방법을 익힙니다.
* **Request/Response Handling:** HTTP 요청을 처리하고, 적절한 상태 코드와 함께 JSON 응답을 반환하는 방법을 배웁니다.
* **Serde 통합:** `serde`를 사용하여 Rust 구조체와 JSON 간의 직렬화/역직렬화(Serialization/Deserialization)를 구현합니다.
* **Error Handling in Web:** 웹 서비스 환경에서 에러를 안전하고 일관되게 처리하는 패턴을 익힙니다.

---

## 2. 세션 일정 (Session Breakdown)

| 시간 | 주제 | 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:30 | **Introduction** | Axum 소개, 왜 Axum인가? (Tokio, Tower, Hyper와의 관계) |
| 00:30 - 01:30 | **Routing & Extractors** | 기본 Router 설정, Path/Query/State Extractor 활용법 |
| 01:30 - 02:30 | **JSON & Serde** | JSON 요청 처리, `serde`를 이용한 데이터 모델링 |
| 02:30 - 03:30 | **Error Handling** | `IntoResponse` 트레이트를 이용한 커스텀 에러 처리 |
| 03:30 - 05:00 | **Hands-on Lab** | 간단한 Task Management REST API 구현 |

---

## 3. 핵심 개념 (Key Concepts)

### 3.1 Axum 프레임워크 구조
Axum은 `hyper`를 기반으로 하며, `tower` 서비스 생태계를 그대로 활용합니다. 이는 미들웨어(Layer)를 매우 쉽게 추가할 수 있음을 의미합니다.

### 3.2 Extractors (추출기)
Axum의 가장 강력한 기능 중 하나입니다. 핸들러 함수의 인자로 특정 타입을 지정하면, Axum이 요청에서 해당 데이터를 자동으로 추출해 줍니다.
* `Path<T>`: URL 경로 파라미터 추출
* `Query<T>`: Query String 파라미터 추출
* `Json<T>`: 요청 Body의 JSON 데이터 추출
* `State<S>`: 공유 상태(DB 커넥션 풀 등) 추출

### 3.3 Shared State (공유 상태)
비동기 서버에서는 여러 요청이 동시에 들어오므로, DB 커넥션 풀이나 설정값 등을 안전하게 공유해야 합니다. Axum에서는 `State` extractor를 통해 이를 처리합니다.

### 3.4 Error Handling (에러 처리)
웹 서버에서 에러는 단순히 `panic!`이 아니라, 클라이언트에게 적절한 HTTP 상태 코드(400, 404, 500 등)와 에러 메시지를 전달하는 과정이어야 합니다. Axum에서는 `IntoResponse` 트레이트를 구현함으로써 이를 우아하게 처리할 수 있습니다.

---

## 4. 데모 포인트 (Demonstration Points)

1.  **Hello World:** 가장 단순한 Axum 서버 실행.
2.  **Extractor 활용:** `/user/:id` 경로에서 `id`를 추출하여 응답하기.
3.  **JSON API:** `POST /tasks` 요청을 받아 JSON 데이터를 파싱하고 응답하기.
4.  **Error Mapping:** 잘못된 JSON 형식이 들어왔을 때 커스텀 에러 메시지와 `400 Bad Request` 반환하기.

---

## 5. 실습 과제 (Lab/Assignment)

### 과제명: 간단한 Task Management API 만들기

**요구사항:**
1.  `Task` 구조체를 정의합니다 (id: u32, title: String, completed: bool).
2.  메모리 내 저장소(`Arc<Mutex<Vec<Task>>>`)를 사용하여 데이터를 관리합니다.
3.  다음 엔드포인트를 구현합니다:
    * `GET /tasks`: 모든 Task 목록 조회
    * `POST /tasks`: 새로운 Task 생성 (JSON Body 사용)
    * `GET /tasks/:id`: 특정 Task 조회
    * `PUT /tasks/:id`: 특정 Task의 완료 여부 업데이트
4.  **주의사항:** `unwrap()` 사용을 지양하고, 에러 발생 시 적절한 HTTP 상태 코드를 반환하도록 구현하세요.

**검증 방법:**
* `cargo run`으로 서버 실행 후 `curl` 또는 Postman을 사용하여 테스트합니다.
* `cargo test`를 통해 작성한 로직이 올바른지 확인합니다.
