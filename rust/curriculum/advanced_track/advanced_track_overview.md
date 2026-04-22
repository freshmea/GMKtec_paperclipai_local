# Rust 고급 트랙 (Advanced Track) 개요

## 1. 트랙 소개
본 고급 트랙은 Rust 숙련도를 높이기 위한 심화 과정으로, **"고성능 시스템 프로그래밍"**과 **"복잡한 소프트웨어 아키텍처 설계"** 능력을 배양하는 것을 목표로 합니다. 

단순히 코드를 작성하는 것을 넘어, 하드웨어 자원을 효율적으로 활용하고, 대규모 동시성 문제를 해결하며, 신뢰할 수 있는 시스템을 구축하는 엔지니어가 되기 위한 과정입니다.

## 2. 학습 철학
- **Why over How:** 특정 라이브러리나 패턴을 사용하는 방법(How)보다, 왜 그 설계가 선택되었는지(Why)를 이해합니다.
- **Performance-First:** 데이터 구조의 메모리 레이아웃, CPU 캐시 효율성, I/O 병목 지점 등 성능에 직접적인 영향을 주는 요소들을 고려합니다.
- **Systems Thinking:** 개별 컴포넌트의 동작뿐만 아니라, 전체 시스템의 생명주기(Lifecycle), 장애 모드(Failure Mode), 그리고 관찰 가능성(Observability)을 통합적으로 사고합니다.

## 3. 대상 학습자
- Rust의 기본 문법(Ownership, Borrow Checker, Traits)에 익숙한 개발자
- 비동기 프로그래밍(async/await)의 기본 개념을 이해하고 있는 개발자
- 시스템 소프트웨어, 고성능 백엔드, 또는 CLI 도구 개발에 관심이 있는 엔지니어

## 4. 학습 로드맵

본 트랙은 총 4개의 핵심 모듈로 구성되어 있으며, 각 모듈은 이론 학습과 프로젝트 기반 실습이 결합된 형태로 진행됩니다.

| 모듈 | 주제 | 핵심 키워드 | 목표 |
| :--- | :--- | :--- | :--- |
| **M1** | **시스템 최적화** | SIMD, Memory Layout, Cache Locality, Zero-copy | 하드웨어 친화적인 고성능 코드 작성 능력 배양 |
| **M2** | **고급 동시성 제어** | Atomics, Lock-free, Memory Ordering, Backpressure | 복잡한 동시성 환경에서의 안정적이고 효율적인 제어 |
| **M3** | **도구 및 아키텍처 설계** | CLI Design, Plugin System, Modular Architecture | 확장 가능하고 유지보수하기 쉬운 시스템 설계 능력 |
| **M4** | **신뢰성 및 관찰 가능성** | Structured Logging, Tracing, Error Modeling | 장애에 강하고 운영 가능한 시스템 구축 |

## 5. 학습 방식
- **Case Study:** `ripgrep`, `uv` 등 세계적인 Rust 오픈소스 프로젝트의 설계와 구현을 심층 분석합니다.
- **Build & Benchmark:** 실제 도구의 핵심 기능을 구현하고, `criterion`을 사용하여 성능을 정량적으로 검증합니다.
- **Reproduce & Debug:** 복잡한 시스템의 버그를 재현하고, 디버깅 도구를 활용하여 근본 원인(Root Cause)을 찾는 연습을 합니다.

---
*본 커리큘럼은 지속적으로 업데이트되며, 최신 Rust 생태계의 트렌드를 반영합니다.*
