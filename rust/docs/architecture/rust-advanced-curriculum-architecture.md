# Rust 고급 커리큘럼 아키텍처 설계

## 1. 설계 철학
본 커리큘럼은 단순한 언어 문법 학습을 넘어, **"실제 고성능 시스템이 어떻게 설계되고 동작하는가"**를 이해하는 데 초점을 맞춘다.

## 2. 교육 트랙 구조
고급 트랙은 다음과 같은 3가지 핵심 축으로 구성된다.

### A. Deep Dive (사례 분석)
- `ripgrep`: 고성능 검색 엔진의 데이터 구조와 I/O 패턴 분석.
- `uv`: 현대적인 패키지 매니저의 비동기 제어와 파일 시스템 오케스트레이션 분석.

### B. Project-Based Learning (PBL)
- 이론을 실제 구현으로 연결하는 모듈형 프로젝트.
- 각 프로젝트는 특정 시스템 문제(예: Race condition, Memory fragmentation)를 해결하도록 설계.

### C. Reliability Engineering (신뢰성 공학)
- Observability (tracing, metrics).
- Error Handling & Recovery.
- Reproducibility (Bug reproduction playbook).

## 3. 기술적 핵심 요소 (Technical Pillars)
- **Zero-cost Abstractions:** Trait 기반 설계와 Generics의 효율적 사용.
- **Memory Safety & Performance:** Unsafe의 제한적 사용과 Ownership/Lifetime의 극한 활용.
- **Concurrency & Async:** Tokio 기반의 비동기 I/O와 데이터 레이스 방지.
- **System Integration:** OS 수준의 리소스(File, Network, Process) 제어.

## 4. 데이터 모델 및 문서화 전략
- 모든 문서는 Markdown으로 작성하며, 코드 예제와 함께 제공된다.
- 설계 근거(Rationale)를 명확히 하여 "왜 이렇게 설계했는가"에 대한 답을 제공한다.
