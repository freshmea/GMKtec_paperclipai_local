# Rust 교육 커리큘럼 산출물 구조

이 디렉토리는 Rust 교육 과정을 위한 강의 스크립트와 실습 코드를 포함하고 있습니다.

## 📂 디렉토리 구조

- `curriculum/`: 일자별/시간대별 상세 강의 스크립트 (Markdown 형식)
    - `advanced_track/`: 고성능 도구 분석 및 프로젝트 중심의 고급 과정 (Case Study, Reproduction Playbook 포함)
- `code/`: 실습용 Rust 예제 프로젝트 및 단일 파일 코드

## 🚀 고급 트랙 (Advanced Track) 안내

본 저장소에는 Rust 숙련도를 높이기 위한 **고급 트랙(Advanced Track)** 커리큘럼이 포함되어 있습니다. 이 과정은 단순한 문법 학습을 넘어, 실제 고성능 시스템 도구의 설계와 구현 원리를 깊이 있게 탐구합니다.

### 📎 바로가기

- 개요: `curriculum/advanced_track/advanced_track_overview.md`
- 프로젝트 모듈: `curriculum/advanced_track/advanced_project_modules.md`
- 사례 분석 `ripgrep`: `curriculum/advanced_track/case_study_ripgrep.md`
- 사례 분석 `uv`: `curriculum/advanced_track/case_study_uv.md`
- 재현 플레이북: `curriculum/advanced_track/reproduction_playbook.md`

### 📚 커리큘럼 구성

| 모듈 | 주제 | 핵심 목표 |
| :--- | :--- | :--- |
| **M1** | **시스템 최적화** | SIMD, 메모리 레이아웃, Zero-copy를 활용한 고성능 코드 작성 |
| **M2** | **고급 동시성 제어** | 커스텀 스케줄러 및 Lock-free 알고리즘을 통한 정밀한 동시성 제어 |
| **M3** | **도구 및 아키텍처 설계** | 확장 가능하고 유지보수하기 쉬운 CLI 및 분산 시스템 설계 |
| **M4** | **신뢰성 및 관찰 가능성** | 구조화된 로깅 및 트레이싱을 활용한 운영 가능한 시스템 구축 |

### 🛠️ 학습 방식

1. **Case Study (사례 분석):** `ripgrep`, `uv` 등 세계적인 Rust 오픈소스 프로젝트의 소스 코드를 분석하여 그들의 설계 철학과 최적화 기법을 배웁니다.
2. **Project-Based Learning (프로젝트 기반 학습):** 분석한 내용을 바탕으로 자신만의 고성능 도구를 직접 구현합니다.
3. **Benchmark-Driven (벤치마크 중심):** `criterion` 등을 사용하여 구현한 기능의 성능을 정량적으로 측정하고 검증합니다.

---

## 📖 사용 방법

### 1. 강의 스크립트 확인
`curriculum/` 디렉토리 내의 파일을 열어 해당 일차의 강의 흐름, 목표, 데모 포인트, 실습 지시 사항을 확인합니다.

### 2. 실습 코드 실행
각 실습 코드는 독립적인 Cargo 프로젝트 또는 단일 파일로 구성되어 있습니다.

#### Cargo 프로젝트인 경우:
```bash
cd code/<project_name>
cargo run
```

#### 단일 파일인 경우:
```bash
rustc <file_name>.rs
./<file_name>
```

### 3. 학습 흐름
1. **이론 세션:** `curriculum/`의 스크립트를 통해 개념을 학습합니다.
2. **데모:** 강사가 제공하는 예제 코드를 실행하며 동작 원리를 이해합니다.
3. **실습:** `code/`에 있는 프로젝트를 직접 타이핑하거나, 제공된 뼈대 코드를 완성합니다.
4. **검증:** `cargo test` 또는 `cargo clippy`를 사용하여 코드의 품질을 확인합니다.

## ⚠️ 주의 사항
- 모든 코드는 `cargo check` 및 `cargo test`가 통과하는 것을 원칙으로 합니다.
- 학습 중 발생하는 컴파일 에러는 Rust 학습의 핵심 과정이므로, 에러 메시지를 꼼꼼히 읽는 습관을 들입니다.
