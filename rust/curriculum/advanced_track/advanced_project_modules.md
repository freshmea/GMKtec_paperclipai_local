# 고급 프로젝트 모듈 (Advanced Project Modules)

본 문서는 고급 트랙의 핵심인 프로젝트 중심 학습(PBL)을 위한 상세 모듈 구성을 정의합니다. 각 모듈은 특정 기술적 도전 과제를 해결하며, 이론적 이해를 실무적인 구현 능력으로 전환하는 데 초점을 맞춥니다.

## 1. 모듈 구성 요약

| 모듈 | 프로젝트 명 | 핵심 학습 목표 | 난이도 |
| :--- | :--- | :--- | :--- |
| **M1** | **High-Perf Search Engine** | SIMD 기반 문자열 검색 및 메모리 맵 I/O 최적화 | 상 |
| **M2** | **Async Task Scheduler** | 커스텀 비동기 런타임 구현 및 Work-stealing 스케줄러 이해 | 최상 |
| **M3** | **Distributed Key-Value Store** | 네트워크 프로토콜(gRPC/TCP) 및 합의 알고리즘 기초 | 상 |
| **M4** | **Observability Sidecar** | Structured Logging 및 Tracing을 활용한 시스템 모니터링 도구 | 중 |

---

## 2. 모듈별 상세 설계

### [M1] High-Perf Search Engine
`ripgrep`의 핵심 메커니즘을 모방하여 대규모 파일 시스템에서 초고속 문자열 검색을 수행하는 도구를 구축합니다.

- **목표:**
    - `mmap`을 활용한 효율적인 파일 읽기 구현
    - SIMD(Single Instruction, Multiple Data)를 이용한 패턴 매칭 가속
    - 멀티스레드 기반의 병렬 파일 탐색
- **선수 지식:**
    - Rust의 `std::fs`, `std::mem`
    - 비동기 I/O 및 멀티스레딩 기초
    - SIMD 명령어 집합에 대한 이해
- **결과물:**
    - `search-engine` CLI 도구
    - 검색 속도 및 메모리 사용량 벤치마크 보고서
- **검증 방법:**
    - `criterion`을 사용한 기존 `grep` 대비 성능 비교
    - 대용량 데이터셋에서의 정확성 테스트

### [M2] Async Task Scheduler
Tokio와 같은 현대적인 비동기 런타임의 핵심인 스케줄러를 직접 구현하여 비동기 프로그래밍의 내부 동작을 이해합니다.

- **목표:**
    - Work-stealing 알고리즘을 적용한 멀티스레드 스케줄러 구현
    - Task의 생명주기 및 Cancellation 메커니즘 제어
    - Backpressure를 처리하는 채널 설계
- **선수 지식:**
    - `Future` 트레이트 및 `Poll` 모델
    - `Pin`, `Unpin` 등 고급 소유권 및 생명주기 개념
    - `std::sync::atomic`을 이용한 동기화
- **결과물:**
    - `mini-tokio` 라이브러리 및 데모 애플리케이션
- **검증 방법:**
    - 다양한 부하 상황에서의 스케줄링 지연 시간(Latency) 측정
    - Race condition 및 Deadlock 방지 테스트

### [M3] Distributed Key-Value Store
네트워크를 통해 데이터를 저장하고 조회하는 분산 시스템의 기초를 다집니다.

- **목표:**
    - `tonic`(gRPC)을 이용한 클라이언트-서버 통신 구현
    - 데이터 일관성을 위한 단순화된 합의 프로토콜 적용
    - 네트워크 장애 상황에서의 재시도 및 타임아웃 전략
- **선수 지식:**
    - 네트워크 프로토콜(TCP/HTTP2)
    - `async/await` 기반의 서버 구현
    - 데이터베이스 인덱싱 기초
- **결과물:**
    - `distri-kv` 서버 및 클라이언트 패키지
- **검증 방법:**
    - 네트워크 단절 시 데이터 복구 및 일관성 테스트
    - 동시 요청에 대한 처리량(Throughput) 측정

### [M4] Observability Sidecar
운영 환경에서 시스템의 상태를 투명하게 관찰할 수 있는 도구를 구축합니다.

- **목표:**
    - `tracing` 크레이트를 활용한 구조화된 로그 생성
    - 메트릭 수집 및 Prometheus 포맷 노출
    - 시스템 이벤트에 대한 실시간 모니터링 대시보드 연동
- **선수 지식:**
    - 로그 레벨 및 구조화된 로깅 개념
    - 기본적인 HTTP/Prometheus 프로토콜
- **결과물:**
    - `obs-sidecar` 에이전트 및 모니터링 대시보드 구성 가이드
- **검증 방법:**
    - 특정 이벤트 발생 시 로그 및 메트릭의 정확한 기록 확인
    - 사이드카 도입 시 시스템 오버헤드 측정

---

## 3. 학습 가이드
- **단계적 접근:** 각 모듈은 "최소 기능 제품(MVP) 구현 $\rightarrow$ 성능 분석 $\rightarrow$ 최적화" 순서로 진행한다.
- **도구 활용:** `cargo clippy`로 코드 품질을 유지하고, `tracing`으로 실행 흐름을 추적한다.
- **기록:** 모든 설계 결정과 성능 개선 수치는 문서화하여 공유한다.

