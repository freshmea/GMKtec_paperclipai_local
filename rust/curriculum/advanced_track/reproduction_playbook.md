# 재현 플레이북 (Reproduction Playbook)

본 문서는 고성능 Rust 도구의 성능을 분석하거나, 특정 이슈(버그 또는 성능 저하)를 재현하기 위한 표준 절차를 정의합니다.

## 1. 재현 목적 정의 (Define Reproduction Goal)

재현을 시작하기 전에 무엇을 확인하고자 하는지 명확히 합니다.
- **성능 이슈:** 특정 작업이 예상보다 느린가? (Throughput, Latency 분석)
- **버그 이슈:** 특정 입력이나 환경에서 잘못된 결과가 나오는가? (Correctness 확인)
- **회귀 테스트:** 이전 버전에서 해결된 문제가 다시 발생하는가?

## 2. 환경 격리 (Environment Isolation)

재현 결과의 신뢰성을 위해 환경을 최대한 통제합니다.

### A. 시스템 환경 기록
- OS 버전, 커널 버전, CPU 아키텍처
- 설치된 도구 버전 (Rust 컴파일러, cargo, 라이브러리 등)
- 파일 시스템 종류 (ext4, APFS, NTFS 등 - 링크 성능에 영향)

### B. 의존성 고정 (Dependency Pinning)
- `Cargo.lock`을 사용하여 의존성 버전을 고정합니다.
- 특정 버전의 Rust toolchain을 사용하도록 `rust-toolchain.toml`을 설정합니다.

## 3. 데이터 세트 준비 (Data Set Preparation)

분석 대상이 되는 데이터를 준비합니다.

- **파일 시스템 테스트:** 대규모 디렉토리 구조, 수백만 개의 작은 파일, 매우 큰 단일 파일 등.
- **입력 데이터:** 정규식 테스트를 위한 다양한 패턴의 텍스트 파일, 복잡한 의존성 그래프를 가진 설정 파일 등.

## 4. 실행 및 측정 (Execution & Measurement)

### A. 프로파일링 및 분석 도구 (Profiling Tools)

| 도구 | 용도 | 주요 명령어 예시 |
| :--- | :--- | :--- |
| **`cargo flamegraph`** | CPU Hotspot 시각화 | `cargo flamegraph -- [args]` |
| **`perf`** | 시스템 레벨 CPU/Event 프로파일링 | `perf record -g -- [command]` |
| **`heaptrack`** | 메모리 할당 패턴 및 누수 분석 | `heaptrack [command]` |
| **`valgrind` (Memcheck)** | 메모리 오류 및 누수 검사 | `valgrind --leak-check=full [command]` |
| **`strace`** | 시스템 콜(syscall) 추적 및 분석 | `strace -c -p [pid]` 또는 `strace [command]` |
| **`criterion`** | 통계적으로 유의미한 벤치마킹 | `cargo bench` |

### B. 반복 실행 및 통계적 유의성
- 단일 실행 결과는 시스템 노이즈(Context switching, Interrupts 등)에 의해 왜곡될 수 있습니다.
- **최소 30회 이상 반복 실행**하여 평균값과 표준 편차를 계산하고, 이상치(Outlier)를 식별합니다.

## 5. 결과 분석 및 보고 (Analysis & Reporting)

수집된 데이터를 바탕으로 다음 항목을 포함하여 보고합니다.

1. **재현 단계 (Steps to Reproduce):** 누구나 동일한 결과를 얻을 수 있도록 상세히 기술.
2. **관찰된 현상 (Observed Behavior):** 기대값과 실제 결과의 차이.
3. **근거 데이터 (Evidence):** 프로파일링 결과(Flamegraph 이미지), 로그, 벤치마크 수치 등.
4. **가설 (Hypothesis):** 문제의 원인에 대한 기술적 추론 (예: "특정 패턴에서 정규식 엔진의 Backtracking 오버헤드가 발생함").
