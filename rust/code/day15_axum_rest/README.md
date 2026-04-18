# Day 15: Axum & RESTful API 실습 가이드

이 프로젝트는 Axum 프레임워크를 사용하여 간단한 Task Management REST API를 구현하는 예제입니다.

## 실행 방법 (How to Run)

1.  **프로젝트 폴더로 이동:**
    ```bash
    cd /home/aa/vllm/rust/code/day15_axum_rest
    ```

2.  **서버 실행:**
    ```bash
    cargo run
    ```
    서버가 `http://127.0.0.1:3000`에서 실행됩니다.

## API 사용법 (API Usage)

### 1. 모든 Task 조회
* **Method:** `GET`
* **URL:** `http://127.0.0.1:3000/tasks`

```bash
curl http://127.0.0.1:3000/tasks
```

### 2. 새로운 Task 생성
* **Method:** `POST`
* **URL:** `http://127.0.0.1:3000/tasks`
* **Body (JSON):** `{"title": "Learn Axum"}`

```bash
curl -X POST http://127.0.0.1:3000/tasks \
     -H "Content-Type: application/json" \
     -d '{"title": "Learn Axum"}'
```

### 3. 특정 Task 조회
* **Method:** `GET`
* **URL:** `http://127.0.0.1:3000/tasks/{id}`

```bash
curl http://127.0.0.1:3000/tasks/1
```

### 4. Task 상태 업데이트 (완료 여부)
* **Method:** `PUT`
* **URL:** `http://127.0.0.1:3000/tasks/{id}`
* **Body (JSON):** `{"completed": true}`

```bash
curl -X PUT http://127.0.0.1:3000/tasks/1 \
     -H "Content-Type: application/json" \
     -d '{"completed": true}'
```

## 학습 포인트 (Learning Points)

* **Axum Router:** 경로(`route`)와 HTTP 메서드(`get`, `post`, `put`)를 어떻게 매핑하는지 확인하세요.
* **Extractors:** `Path`, `State`, `Json`이 어떻게 함수의 인자로 들어오는지 관찰하세요.
* **Shared State:** `Arc<AppState>`를 통해 여러 요청 간에 데이터가 어떻게 안전하게 공유되는지 이해하세요.
* **Error Handling:** 존재하지 않는 ID를 조회했을 때 `404 Not Found`가 반환되는지 확인하세요.
