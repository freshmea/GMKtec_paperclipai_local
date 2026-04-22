# Day 30: Final Project & Review (최종 프로젝트 및 복습)

## 학습 목표
- 한 달 동안 배운 모든 핵심 개념(Ownership, Concurrency, Async, Error Handling, Web, Observability 등)을 통합하여 실제 서비스 수준의 애플리케이션 설계 및 구현
- 코드 리뷰 및 리팩토링을 통한 시스템 안정성 및 성능 최적화 경험

## 세션 구성 (총 6시간)

### 1. 프로젝트 기획 및 설계 (60분)
- **주제 선정**: 간단한 분산 시스템(예: 분산 Key-Value Store, API Gateway, 또는 실시간 채팅 서버)
- **아키텍처 설계**:
    - 데이터 모델링 (Structs, Enums)
    - 통신 방식 (gRPC 또는 REST)
    - 비동기 처리 전략 (Tokio)
    - 에러 핸들링 및 로깅 전략 (anyhow, tracing)

### 2. 핵심 기능 구현 (180분)
- **Phase 1**: 비동기 네트워크 레이어 및 프로토콜 구현
- **Phase 2**: 상태 관리 및 동시성 제어 (Arc, Mutex, RwLock)
- **Phase 3**: 에러 핸들링 및 Observability 적용

### 3. 테스트 및 안정화 (60분)
- Unit Tests 및 Integration Tests 작성
- 경계 조건(Edge cases) 및 동시성 이슈(Race conditions) 확인
- `cargo test`를 통한 검증

### 4. 최종 발표 및 회고 (60분)
- 구현된 코드 시연
- 설계 의도 및 기술적 결정 사항 공유
- 한 달간의 학습 과정에 대한 회고

## 과제
- 구현한 프로젝트를 GitHub 리포지토리에 업로드하고, `README.md`에 설계 문서와 실행 방법을 상세히 작성하십시오.

## 핵심 키워드
- `System Design`, `Integration`, `Refactoring`, `Testing`, `Architecture`
