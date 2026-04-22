# Rust 고급 커리큘럼 프로젝트 계획

## 1. 개요
본 프로젝트는 Rust 숙련도를 높이기 위한 고급 트랙 커리큘럼을 설계하고, 실제 고성능 도구(`ripgrep`, `uv`)의 사례 분석을 통해 실전적인 엔지니어링 인사이트를 제공하는 것을 목표로 한다.

## 2. 목표 및 산출물
### 목표
- 프로젝트 중심의 고급 Rust 학습 경로 설계
- 고성능 Rust 도구의 설계 패턴 및 성능 최적화 전략 분석
- 복잡한 시스템의 버그 재현 및 테스트 전략 가이드 작성

### 필수 산출물
1. **개요:** `/home/aa/vllm/rust/curriculum/advanced_track/advanced_track_overview.md`
2. **프로젝트 모듈:** `/home/aa/vllm/rust/curriculum/advanced_track/advanced_project_modules.md`
3. **사례 분석 (ripgrep):** `/home/aa/vllm/rust/curriculum/advanced_track/case_study_ripgrep.md`
4. **사례 분석 (uv):** `/home/aa/vllm/rust/curriculum/advanced_track/case_study_uv.md`
5. **재현 플레이북:** `/home/aa/vllm/rust/curriculum/advanced_track/reproduction_playbook.md`
6. **README 업데이트:** `/home/aa/vllm/rust/README.md`

## 3. 상세 작업 단계

### 단계 1: 기반 설계 및 계획
- [ ] 프로젝트 계획 수립 (`plans/2026-04-22-rust-advanced-curriculum-plan.md`)
- [ ] 설계 메모 작성 (`$AGENT_HOME/architecture/rust-advanced-curriculum-architecture.md`)
- [ ] 관리 메모 작성 (`$AGENT_HOME/docs/rust-advanced-curriculum-management.md`)

### 단계 2: 고급 트랙 커리큘럼 설계
- [ ] `advanced_track_overview.md`: 전체 트랙의 철학, 대상, 학습 로드맵 정의
- [ ] `advanced_project_modules.md`: 각 모듈별 목표, 선수 지식, 결과물, 검증 방법 정의

### 단계 3: 사례 분석 (Case Studies)
- [ ] `case_study_ripgrep.md`: 
    - 검색 알고리즘, 메모리 레이아웃, SIMD 활용, I/O 최적화 분석
    - 문제 정의, 구조, trade-off, 재현 전략 포함
- [ ] `case_study_uv.md`:
    - 패키지 매니저의 구조, 병렬성, 파일 시스템 최적화, 워크플로우 분석
    - 문제 정의, 구조, trade-off, 재현 전략 포함

### 단계 4: 운영 및 검증 가이드
- [ ] `reproduction_playbook.md`: 시스템 레벨의 버그 재현, 테스트 자동화, observability 전략

### 단계 5: 최종 정리
- [ ] `README.md` 업데이트: 고급 트랙 및 사례 분석 섹션 추가
- [ ] 최종 검토 및 완료 보고 (CHO-80에 코멘트)

## 4. 기술 스택 및 참고 사항
- **언어:** Rust (Stable)
- **분석 대상:** `ripgrep` (grep/regex/SIMD), `uv` (async/io/package management)
- **작성 원칙:** 한국어 사용, 실전 중심, 설계 근거 명시
