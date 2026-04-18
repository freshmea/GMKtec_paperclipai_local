# Day 18: Observability (Tracing & Metrics)

## 학습 목표 (Objectives)
본 세션의 목표는 Rust 애플리케이션의 실행 상태를 추적하고 성능 지표를 수집하는 방법을 배우는 것입니다. 시스템의 복잡성이 증가함에 따라, 단순히 로그를 남기는 것을 넘어 구조화된 데이터(Structured Data)를 통해 시스템의 내부 동작을 관찰하는 능력이 필수적입니다.

1. **Tracing**의 개념과 필요성 이해
2. `tracing` 크레이트를 사용하여 구조화된 로깅 및 Span 관리 수행
3. **Metrics**의 개념과 시스템 성능 모니터링 방법 이해
4. `metrics` 크레이트를 사용하여 커스텀 메트릭(Counter, Gauge, Histogram) 수집 방법 습득

## 세션 타임라인 (Session Breakdown)
- **09:00 - 10:30**: Observability 기초 이론 (Logging vs Tracing vs Metrics)
- **10:30 - 12:00**: `tracing` 크레이트 실습: Span, Event, Subscriber 설정
- **13:00 - 14:30**: Context Propagation 및 Distributed Tracing 개념
- **14:30 - 16:00**: `metrics` 크레이트 실습: Counter, Gauge, Histogram 구현
- **16:00 - 18:00**: 종합 실습: Axum 웹 서버에 Tracing 및 Metrics 통합하기

## 핵심 개념 (Key Concepts)

### 1. Logging vs Tracing
- **Logging**: 특정 시점에 발생한 개별 이벤트를 기록합니다. (예: "User logged in")
- **Tracing**: 요청(Request)의 전체 생명주기를 추적합니다. 하나의 요청이 여러 함수와 서비스를 거치는 과정을 `Span`이라는 단위로 묶어 흐름을 파악합니다.

### 2. Structured Logging
단순 텍스트 로그 대신, JSON과 같은 구조화된 형식으로 데이터를 남깁니다. 이를 통해 로그 분석 도구(ELK Stack, Grafana Loki 등)에서 필터링과 쿼리가 용이해집니다.

### 3. Metrics
시스템의 상태를 나타내는 수치 데이터입니다.
- **Counter**: 계속 증가하는 값 (예: 총 요청 횟수)
- **Gauge**: 현재 상태를 나타내는 값 (예: 현재 메모리 사용량, 동시 접속자 수)
- **Histogram**: 값의 분포를 나타내는 데이터 (예: HTTP 응답 시간 레이턴시)

## 데모 포인트 (Demonstration Points)
- `tracing-subscriber`를 사용하여 콘솔에 구조화된 로그 출력하기
- `#[tracing::instrument]` 매크로를 사용하여 함수 호출에 자동으로 Span 생성하기
- `metrics-exporter-prometheus`를 사용하여 Prometheus 형식으로 메트릭 노출하기

## 실습 과제 (Lab/Assignment)

### 과제 1: Tracing 적용
제공된 `day18_observability` 프로젝트의 `src/main.rs`를 수정하여 다음을 수행하세요.
1. `tracing_subscriber`를 설정하여 로그를 출력합니다.
2. `process_request` 함수에 `#[tracing::instrument]`를 적용합니다.
3. 함수 내부에서 `tracing::info!`와 `tracing::error!`를 사용하여 이벤트(Event)를 남깁니다.

### 과제 2: Metrics 수집
1. `metrics` 크레이트를 사용하여 `http_requests_total` 카운터를 만듭니다.
2. 요청이 처리될 때마다 카운터를 증가시키세요.
3. 요청 처리 시간을 측정하기 위해 `histogram!`을 사용하여 레이턴시를 기록하세요.

## 정리 (Summary)
Observability는 "시스템이 왜 이렇게 동작하는가?"에 대한 답을 주는 도구입니다. `tracing`으로 흐름을 보고, `metrics`로 상태를 숫자로 확인하는 습나다.
