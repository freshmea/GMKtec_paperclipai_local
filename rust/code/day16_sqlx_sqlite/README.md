# Day 16: SQLx & SQLite 실습 가이드

이 프로젝트는 Axum 프레임워크와 SQLx를 사용하여 SQLite 데이터베이스와 연동되는 사용자 관리 API를 구현하는 예제입니다.

## 실행 방법 (How to Run)

1.  **프로젝트 폴더로 이동:**
    ```bash
    cd /home/aa/vllm/rust/code/day16_sqlx_sqlite
    ```

2.  **서버 실행:**
    ```bash
    cargo run
    ```
    *이 예제는 메모리 내 SQLite(`sqlite::memory:`)를 사용하므로 서버를 재시작하면 데이터가 초기화됩니다.*

## API 사용법 (API Usage)

### 1. 모든 사용자 조회
* **Method:** `GET`
* **URL:** `http://127.0.0.1:3000/users`

```bash
curl http://127.0.0.1:3000/users
```

### 2. 새로운 사용자 등록
* **Method:** `POST`
* **URL:** `http://127.0.0.1:3000/users`
* **Body (JSON):** `{"name": "Alice", "email": "alice@example.com"}`

```bash
curl -X POST http://127.0.0.1:3000/users \
     -H "Content-Type: application/json" \
     -d '{"name": "Alice", "email": "alice@example.com"}'
```

### 3. 특정 사용자 조회
* **Method:** `GET`
* **URL:** `http://127.0.0.1:3000/users/{id}`

```bash
curl http://127.0.0.1:3000/users/1
```

## 학습 포인트 (Learning Points)

* **`sqlx::query_as`:** SQL 쿼리 결과를 Rust 구조체로 자동 매핑하는 방법을 확인하세요.
* **`AppState` & `SqlitePool`:** 데이터베이스 연결 풀을 어떻게 `State`로 관리하고 핸들러에 전달하는지 이해하세요.
* **비동기 DB 작업:** `await`를 통해 DB I/O가 비동기적으로 처리되는 과정을 관찰하세요.
* **에러 매핑:** DB 에러(`sqlx::Error`)를 HTTP 상태 코드(`StatusCode`)로 변환하는 패턴을 살펴보세요.
