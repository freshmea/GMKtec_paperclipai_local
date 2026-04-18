# Day 16: 데이터베이스 통합 (Database Integration with SQLx)

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust 애플리케이션에서 데이터베이스를 안전하고 효율적으로 다루는 방법을 배우는 것입니다. 특히 **SQLx**를 사용하여 컴파일 타임에 SQL 쿼리를 검증하고, 비동기 방식으로 데이터베이스와 상호작용하는 방법을 익힙니다.

* **데이터베이스 핵심 개념:** Connection Pool, Migrations, Prepared Statements의 이해.
* **SQLx 라이브러리 활용:** `sqlx`를 사용하여 비동기 SQL 쿼리 실행.
* **Compile-time SQL Verification:** `query!` 매크로를 통한 쿼리 문법 및 타입 검증.
* **데이터 모델링:** 데이터베이스 테이블과 Rust 구조체 간의 매핑 (Serde 및 SQLx 매크로).
* **Migration Workflow:** `sqlx-cli`를 이용한 스키마 변경 관리.

---

## 2. 세션 일정 (Session Breakdown)

| 시간 | 주제 | 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:40 | **DB & Async I/O** | 왜 비동기 DB 드라이버가 필요한가? Connection Pooling의 중요성 |
| 00:40 - 01:30 | **SQLx 기초** | `sqlx::connect`, `query!`, `fetch_one`, `fetch_all` 사용법 |
| 01:30 - 02:30 | **Compile-time Safety** | `DATABASE_URL` 환경 변수와 컴파일 타임 검증의 원리 |
| 02:30 - 03:30 | **Migrations** | 스키마 버전 관리 및 `sqlx-cli`를 이용한 마이그레이션 실행 |
| 03:30 - 05:00 | **Hands-on Lab** | Axum API에 PostgreSQL(또는 SQLite) 연동하기 |

---

## 3. 핵심 개념 (Key Concepts)

### 3.1 SQLx의 특징
SQLx는 "Pure Rust" 비동기 SQL 라이브러리로, 다음과 같은 강력한 기능을 제공합니다.
* **Compile-time checking:** 쿼리가 실제 DB 스키마와 일치하는지 컴파일 시점에 확인합니다.
* **No ORM overhead:** ORM(Object-Relational Mapping)의 복잡함 없이 직접 SQL을 작성하면서도 타입 안전성을 확보합니다.
* **Async-first:** `tokio` 기반의 비동기 런타임과 완벽하게 통합됩니다.

### 3.2 Connection Pool
데이터베이스 연결을 매번 생성하고 닫는 것은 매우 비용이 많이 드는 작업입니다. `SqlitePool`이나 `PgPool`을 사용하여 미리 생성된 연결들을 재사용함으로써 성능을 극대화합니다.

### 3.3 Migrations (마이그레이션)
애플리케이션의 데이터베이스 스키마가 변경될 때, 이를 코드와 함께 버전 관리해야 합니다. SQLx는 `.sql` 파일을 통해 스키마 변경 이력을 관리하는 기능을 제공합니다.

---

## 4. 데모 포인트 (Demonstration Points)

1.  **Basic Query:** 단순 `SELECT` 쿼리를 실행하여 데이터를 구조체로 가져오기.
2.  **Compile Error Demo:** DB 스키마에 없는 컬럼을 `query!`로 조회할 때 발생하는 컴파일 에러 확인.
3.  **Migration Demo:** 새로운 테이블을 추가하는 마이그레이션 파일 생성 및 적용.
4.  **Integration:** 기존 Axum API에 DB 연동하여 `AppState`에 `Pool` 주입하기.

---

## 5. 실습 과제 (Lab/Assignment)

### 과제명: SQLite 기반의 사용자 관리 시스템 구축

**요구사항:**
1.  `sqlx`와 `sqlite` 기능을 사용하여 프로젝트를 구성합니다.
2.  `users` 테이블을 생성하는 마이그레이션 파일을 작성합니다 (id, name, email).
3.  다음 기능을 구현합니다:
    * `POST /users`: 새로운 사용자 등록
    * `GET /users`: 전체 사용자 목록 조회
    * `GET /users/:id`: 특정 사용자 조회
4.  **주의사항:** 모든 DB 작업은 `sqlx::query!` 매크로를 사용하여 컴파일 타임 검증을 수행해야 합니다.

**검증 방법:**
* `cargo run`으로 서버 실행 후 `curl`을 사용하여 데이터가 DB에 정상적으로 저장되고 조회되는지 확인합니다.
* 마이그레이션이 정상적으로 적용되었는지 확인합니다.
