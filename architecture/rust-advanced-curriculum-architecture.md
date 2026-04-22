# Rust 고급 커리큘럼 아키텍처 설계

## 1. 설계 철학
본 커리큘럼은 단순한 문법 학습을 넘어, 실제 고성능 시스템 도구를 분석하고 구현함으로써 **"엔지니어링 사고방식"**을 기르는 것을 목표로 한다.

### 핵심 원칙
- **Deep Dive over Surface Level:** API 사용법이 아니라, 왜 그렇게 설계되었는지(Why)를 파고든다.
- **Performance-Oriented:** 메모리 레이아웃, CPU 캐시 효율성, I/O 병목 지점을 이해한다.
- **Correctness & Safety:** Rust의 타입 시스템을 활용하여 런타임 에러를 설계 단계에서 방지하는 방법을 배운다.
- **Observability:** 시스템의 내부 상태를 어떻게 측정하고 관찰할 것인지 고려한다.

## 2. 교육 트랙 구조

### 고급 트랙 (Advanced Track)
기존의 기초/중급 과정을 마친 학습자를 대상으로 하며, 다음과 같은 주제를 다룬다.

1. **시스템 최적화 (System Optimization)**
   - SIMD, 메모리 정렬, 캐시 친화적 데이터 구조
   - 비동기 I/O 및 컨텍스트 스위칭 비용 최소화
2. **복잡한 동시성 제어 (Advanced Concurrency)**
   - Lock-free 데이터 구조, Atomics, Memory Ordering
   - 비동기 런타임 내부 동작 및 Backpressure 제어
3. **도구 설계 및 구현 (Tooling Design)**
   - CLI 아키텍처, 명령줄 인터페이스 설계 원칙
   - 패키지 매니저 및 빌드 시스템의 워크플로우 설계
4. **신뢰성 및 관찰 가능성 (Reliability & Observability)**
   - Structured Logging, Tracing, Metrics
   - 에러 모델 설계 및 Graceful Shutdown 전략

## 3. 사례 분석 대상 (Case Studies)

### ripgrep (`rg`)
- **Focus:** 고성능 문자열 검색 및 정규표현식 매칭
- **Key Concepts:** SIMD 가속, 메모리 맵(mmap), 효율적인 정규표현식 엔진 활용

### uv (Python Package Manager)
- **Focus:** 초고속 패키지 관리 및 가상 환경 구축
- **Key Concepts:** 비동기 파일 I/O, 병렬 다운로드/설치, 파일 시스템 캐싱 및 링크(reflink) 활용

## 4. 검증 및 실습 전략
- **Project-Based Learning:** 각 모듈은 실제 작동하는 도구의 부분 구현을 포함한다.
- **Benchmark-Driven:** 구현한 기능의 성능을 `criterion` 등을 통해 직접 측정한다.
- **Code Review Simulation:** 실제 오픈소스 프로젝트의 PR을 분석하고 리뷰하는 연습을 병행한다.
