# Rust 성공 프로젝트 카탈로그

본 문서는 고급 트랙에서 참고할 Rust 성공 프로젝트를 추가로 정리한 자료다. 기준은 단순 인기보다 학습 가치가 분명한가, 설계 trade-off가 드러나는가, 실제 재현 프로젝트로 축소할 수 있는가에 두었다.

## 선정 기준

- 공식 문서와 저장소 기준으로 현재도 활발히 사용되거나 유지되는 프로젝트
- 특정 기술 축을 대표하는 프로젝트
- 학습자가 설계를 읽고 작은 프로젝트로 재현할 수 있는 프로젝트

## 프로젝트 목록

### 1. ripgrep

- 분야: CLI 검색, 파일 시스템 순회, 정규식
- 공식 소스: `https://github.com/BurntSushi/ripgrep`
- 학습 포인트:
  - `.gitignore` 기반 탐색 제외
  - 병렬 디렉토리 순회
  - literal search와 regex 경로 분리
- 연결 모듈:
  - M1 High-Perf Search Engine

### 2. uv

- 분야: 패키지 관리, resolver, 캐시, 개발자 경험
- 공식 소스: `https://github.com/astral-sh/uv`
- 학습 포인트:
  - PubGrub 기반 의존성 해결
  - 캐시와 링크 전략
  - 빠른 CLI 경험 설계
- 연결 모듈:
  - M3 Tooling Architecture

### 3. Tokio

- 분야: async runtime, 스케줄링, 네트워크 인프라
- 공식 소스: `https://tokio.rs/`
- 학습 포인트:
  - work-stealing scheduler
  - async I/O, timer, sync primitives
  - 운영 환경을 고려한 runtime abstraction
- 연결 모듈:
  - M2 Async Task Scheduler
  - M4 Observability Sidecar

### 4. Serde

- 분야: 데이터 모델링, serialization, zero-cost abstraction
- 공식 소스: `https://serde.rs/`
- 학습 포인트:
  - trait 중심 확장 구조
  - derive와 수동 구현의 경계
  - reflection 없이 generic serialization 달성
- 연결 모듈:
  - M3 Tooling Architecture

### 5. rustls

- 분야: TLS, 보안, 프로토콜 구현
- 공식 소스: `https://github.com/rustls/rustls`
- 학습 포인트:
  - 메모리 안전한 보안 프로토콜 구현
  - 작은 API 표면과 명확한 책임 분리
  - 네트워크 스택 통합 지점 설계
- 연결 모듈:
  - M4 Observability and Reliability

### 6. fd

- 분야: 파일 탐색, 사용자 친화적 CLI
- 공식 소스: `https://github.com/sharkdp/fd`
- 학습 포인트:
  - 병렬 디렉토리 순회
  - 직관적 UX와 sensible defaults
  - ignore 규칙과 검색 속도의 균형
- 연결 모듈:
  - M1 High-Perf Search Engine
  - M3 Tooling Architecture

### 7. bat

- 분야: 개발자 도구 UX, 렌더링, 구성 가능 CLI
- 공식 소스: `https://github.com/sharkdp/bat`
- 학습 포인트:
  - syntax highlighting 파이프라인
  - 설정 파일과 확장성
  - 미려한 CLI UX가 adoption에 미치는 영향
- 연결 모듈:
  - M3 Tooling Architecture

### 8. Tauri

- 분야: 데스크톱 앱 플랫폼, 보안, 크로스 플랫폼
- 공식 소스: `https://v2.tauri.app/`
- 학습 포인트:
  - native web renderer 활용
  - 작은 배포 크기와 보안 중심 설계
  - Rust 백엔드와 웹 프런트엔드 결합
- 연결 모듈:
  - M3 Tooling Architecture
  - M4 Observability and Reliability

### 9. Deno

- 분야: 런타임, 보안 기본값, 대규모 Rust 제품
- 공식 소스: `https://github.com/denoland/deno`
- 학습 포인트:
  - secure defaults
  - Rust + V8 + Tokio 통합 구조
  - 런타임 수준 아키텍처 분해
- 연결 모듈:
  - M2 Async Task Scheduler
  - M3 Tooling Architecture

## 커리큘럼 적용 원칙

- 모든 프로젝트를 깊게 다루지 않는다. 핵심 사례는 `ripgrep`, `uv`로 유지한다.
- 추가 프로젝트는 각 모듈의 비교 사례와 확장 reading으로 배치한다.
- 학습자는 각 모듈에서 최소 1개 핵심 사례, 1개 비교 사례를 읽도록 설계한다.

## 추천 학습 순서

1. `ripgrep`로 CLI 성능 최적화 감각 익히기
2. `fd`, `bat`로 CLI UX와 defaults 설계 비교하기
3. `Tokio`, `Deno`로 런타임과 async 아키텍처 읽기
4. `uv`, `Serde`, `Tauri`로 toolchain과 product architecture 비교하기
5. `rustls`로 안정성과 보안 설계 관점 보강하기
