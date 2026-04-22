# 작업 로그 (Work Log)

## 진행 상황 (Progress)

- [x] 프로젝트 개요 및 로드맵 작성 (advanced_track_overview.md 업데이트)
- [x] 고급 프로젝트 모듈 상세 설계 (advanced_project_modules.md 업데이트)
- [x] Case Study: ripgrep 분석 및 작성 (case_study_ripgrep.md 작성/보완)
- [x] Case Study: uv 분석 및 작성 (case_study_uv.md 작성/보완)
- [ ] 재현 플레이북 작성 (reproduction_playbook.md 작성/보완)
- [ ] README.md 업데이트

## 상세 계획 (Detailed Plan)

### 1. 커리큘럼 구조화 및 로드맵 구체화
- 기존 `advanced_track_overview.md`와 `advanced_project_modules.md`를 검토하여, 학습자가 어떤 순서로 프로젝트를 진행해야 할지 로드맵을 더 명확히 합니다.
- 각 모듈의 난이도와 요구되는 선수 지식을 명시합니다.

### 2. 사례 연구 (Case Studies) 심화
- `ripgrep`과 `uv`의 사례 연구를 단순한 요약을 넘어, 실제 코드의 핵심 로직(예: `ripgrep`의 `ignore` 크레이트 활용 방식, `uv`의 캐싱 전략)을 언급하며 깊이 있게 작성합니다.
- 각 사례 연구에서 "왜 Rust가 이 도구에 최적이었는가?"에 대한 답을 기술적으로 제시합니다.

### 3. 재현 플레이북 (Reproduction Playbook) 완성
- 실제 개발자가 성능 병목을 찾거나 버그를 재현할 때 사용할 수 있는 구체적인 명령어와 도구(예: `flamegraph`, `perf`) 사용법을 포함합니다.

### 4. 최종 검토 및 통합
- 모든 문서의 톤앤매너를 맞추고, `README.md`를 통해 전체 고급 트랙의 진입점을 명확히 합니다.
