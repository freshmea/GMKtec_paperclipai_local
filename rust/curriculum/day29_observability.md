# Day 29: Observability (관측 가능성)

## 학습 목표
- 시스템의 상태를 파악하기 위한 Observability의 핵심 개념(Metrics, Logging, Tracing) 이해
- Rust 생태계의 표준 Observability 도구인 `tracing` 라이브러리 활용법 습득
- Structured Logging을 통한 효율적인 디버깅 및 모니터링 기법 실습

## 세션 구성 (총 4시간)

### 1. Observability 개요 (45분)
- **Monitoring vs Observability**: 단순 상태 확인과 내부 상태 유추의 차이
- **Three Pillars of Observability**:
    - **Logs**: 발생한 이벤트의 기록 (Structured vs Unstructured)
    - **Metrics**: 수치화된 데이터 (Counter, Gauge, Histogram)
    - **Traces**: 요청의 흐름을 추적하는 분산 트레이싱 (Span, Context propagation)

### 2. Structured Logging with `tracing` (60분)
- `log` crate와 `tracing` crate의 차이점
- **Spans**: 작업의 논리적 단위와 생명주기 관리
- **Events**: Span 내부에서 발생하는 개별적인 로그 기록
- **Fields**: 로그에 컨텍스트(ID, User 정보 등)를 포함시키는 방법

### 3. Metrics & Tracing 실습 (75분)
- `tracing-subscriber`를 이용한 로그 레이어 설정
- `metrics` crate를 활용한 시스템 메트릭 수집
- 분산 시스템 환경에서의 Trace ID 전파 개념 이해

### 4. 실습 및 과제 (60분)
- **Lab**: 간단한 HTTP 서버에 `tracing`을 적용하여 요청별 Span 생성 및 Context 유지하기
- **Assignment**: 특정 함수 호출의 실행 시간을 측정하는 Custom Subscriber 또는 Metric 작성

## 핵심 키워드
- `tracing`, `tracing-subscriber`, `metrics`, `structured logging`, `span`, `event`, `context`

## 실습 가이드
- `cargo run`을 실행하여 로그가 어떻게 계층적으로 출력되는지 확인하십시오.
- 로그에 포함된 `request_id`가 동일한 요청 흐름 내에서 어떻게 유지되는지 관찰하십시오.
