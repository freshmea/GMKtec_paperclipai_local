# Day 18: Observability (Tracing & Metrics) 실습 가이드

이 프로젝트는 Rust에서 시스템의 상태를 관찰하기 위한 **Tracing**과 **Metrics**를 구현하는 방법을 배웁니다.

## 실습 목표
1. `tracing` 크레이트를 사용하여 구조화된 로그와 Span을 생성합니다.
2. `metrics` 크레이트를 사용하여 Counter와 Histogram 메트릭을 수집합니다.
3. `metrics-exporter-prometheus`를 사용하여 수집된 메트릭을 HTTP 엔드포인트로 노출합니다.

## 실행 방법

### 1. 프로젝트 실행
```bash
cargo run
```

실행하면 다음과 같은 로그가 출력됩니다:
- `tracing`에 의해 생성된 Span 정보
- `metrics`에 의해 수집된 데이터가 Prometheus 형식으로 출력되는 모습

### 2. 메트릭 확인
서버가 실행 중인 동안 다른 터미널에서 다음 명령을 실행하여 Prometheus 메트릭을 확인할 수 있습니다:
(이 예제에서는 간단하게 표준 출력으로 메트릭을 확인하도록 구현되어 있습니다.)

## 주요 개념
- **Tracing Span**: 작업의 시작과 끝을 나타내는 범위입니다.
- **Counter**: 값이 증가만 하는 메트릭 (예: 요청 횟수).
- **Histogram**: 값의 분포를 측정하는 메트릭 (예: 응답 시간).
