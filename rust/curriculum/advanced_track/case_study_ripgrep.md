# 사례 연구: ripgrep (Case Study: ripgrep)

`ripgrep` (`rg`)은 Rust로 작성된 매우 빠른 텍스트 검색 도구입니다. 이 사례 연구에서는 `ripgrep`이 어떻게 압도적인 성능을 달성하는지 분석합니다.

## 1. 문제 정의 (Problem Definition)

기존의 `grep`이나 `ack`와 같은 도구들은 대규모 코드베이스에서 검색 시 다음과 같은 문제에 직면합니다:
- **I/O 병목:** 디스크에서 파일을 읽어오는 속도가 검색 엔진의 처리 속도를 따라가지 못함.
- **불필요한 파일 검색:** `.gitignore`나 숨김 파일 등 검색할 필요가 없는 파일을 탐색하여 자원 낭비.
- **정규식 처리 성능:** 복잡한 정규식 패턴을 매칭할 때 발생하는 CPU 오버헤드.

## 2. 아키텍처 및 구조 (Architecture & Structure)

`ripgrep`은 성능을 극대화하기 위해 다음과 같은 구조적 설계를 채택합니다:

### A. 병렬 파일 탐색 (Parallel Directory Traversal)
- **`ignore` 크레이트 활용:** `.gitignore`, `.ignore`, `.rgignore` 파일을 파싱하여 검색 대상에서 제외할 규칙을 효율적으로 관리합니다.
- **Work Stealing 패턴:** `rayon` 또는 자체 스레드 풀을 사용하여 디렉토리 탐색 작업을 작은 단위로 나누고, 유휴 스레드가 다른 스레드의 작업을 가져가도록 설계하여 CPU 코어 활용도를 극대화합니다.

### B. 검색 엔진 계층 (Search Engine Layer)
- **`regex` 크레이트의 최적화:** Finite Automata(DFA/NFA) 기반 엔진을 사용하여 정규식 매칭 시 최악의 경우에도 선형 시간 복잡도를 보장하며, ReDoS(Regular Expression Denial of Service) 공격을 원천 차단합니다.
- **SIMD 가속 (Single Instruction, Multiple Data):** 단순 문자열 검색 시 CPU의 SIMD 명령어를 사용하여 한 번의 사이클에 여러 바이트를 동시에 비교함으로써 처리 속도를 비약적으로 높입니다.
- **Literal Search 최적화:** 정규식이 아닌 단순 문자열의 경우, Boyer-Moore 또는 Horspool 알고리즘과 같은 고속 검색 알고리즘을 적용하여 불필요한 비교를 최소화합니다.

### C. 효율적인 버퍼 관리 (Efficient Buffer Management)
- **Memory Mapping (mmap) vs Streaming I/O:** 대용량 파일의 경우 `mmap`을 사용하여 OS 커널의 페이지 캐시를 직접 활용하거나, 적절한 크기의 고정 버퍼를 사용하여 시스템 콜 오버헤드를 줄이고 I/O와 CPU 연산을 오버랩(overlap)시킵니다.

## 3. 기술적 트레이드오프 (Technical Trade-offs)

| 결정 사항 | 이점 (Pros) | 비용 (Cons) |
| :--- | :--- | :--- |
| **Rust 사용** | 메모리 안전성 및 Zero-cost abstraction을 통한 극강의 성능 | C/C++에 비해 컴파일 시간이 길고 학습 곡선이 높음 |
| **`regex` 크레이트 사용** | 검증된 고성능 엔진 활용, ReDoS 공격 방지 | Backtracking 기반의 일부 복잡한 정규식 기능 제한 |
| **병렬 처리 중심 설계** | 멀티코어 CPU 자원 극대화 | 스레드 간 동기화 및 작업 할당(Work stealing) 오버헤드 발생 가능 |

## 4. 재현 및 성능 측정 전략 (Reproduction & Benchmarking)

`ripgrep`의 성능을 직접 측정하고 분석하기 위한 전략입니다:

### A. 환경 준비
- 대규모 소스 코드 저장소 (예: Linux Kernel, Chromium) 준비.
- `criterion` 크레이트를 사용하여 벤치마크 환경 구축.

### B. 측정 지표
- **Throughput:** 초당 처리하는 바이트 수 (MB/s).
- **Latency:** 검색 시작부터 결과 반환까지의 시간.
- **CPU Utilization:** 검색 중 CPU 코어 사용률 및 효율성.

### C. 테스트 시나리오
1. **단순 문자열 검색:** 정규식 없이 특정 키워드만 찾을 때의 성능.
2. **복잡한 정규식 검색:** CPU 집약적인 패턴 매칭 시의 성능.
3. **대규모 디렉토리 스캔:** 파일 개수가 매우 많을 때의 탐색 성능.

## 5. 축소판 재현 프로젝트 설계 (Mini Reproduction Project)

`ripgrep` 전체를 복제하는 대신, 학습용으로는 핵심 경로만 잘라낸 축소판 프로젝트가 더 적절합니다.

### A. 프로젝트 범위
- 입력: 디렉토리 경로, 검색 패턴, ignore 파일 경로
- 출력: 매칭된 파일 경로와 줄 번호
- 제외: 전체 정규식 엔진 구현, 모든 플랫폼 최적화, 전체 CLI 옵션 호환

### B. 구현 단계
1. 단일 스레드 파일 순회기 작성
2. `.gitignore` 유사 규칙을 읽는 간단한 필터 추가
3. literal search 기반의 빠른 검색 경로 추가
4. 멀티스레드 디렉토리 순회로 확장
5. 벤치마크를 붙여 병목을 비교

### C. 학습 포인트
- I/O와 CPU 병목이 각각 어디서 발생하는지 분리해서 본다.
- 검색 알고리즘보다 파일 탐색과 ignore 처리도 전체 성능에 큰 영향을 준다는 점을 확인한다.
- 병렬화는 항상 이득이 아니라는 점을 데이터셋별로 검증한다.
