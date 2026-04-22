# Day 23 실습 가이드: Advanced Concurrency (Rayon & Data Parallelism)

이 가이드는 기본적인 스레드와 채널을 넘어, 데이터 병렬 처리(Data Parallelism)를 효율적으로 수행하기 위한 라이브러리인 `Rayon`을 사용하는 방법을 배웁니다.

## 1. 목표
- `Rayon` 라이브러리를 사용하여 반복문(Iterator)을 병렬로 실행하는 방법을 익힌다.
- `par_iter()`를 통한 데이터 병렬 처리의 개념을 이해한다.
- 'Work Stealing' 알고리즘의 개념을 파악한다.

## 2. 프로젝트 구조
```text
day23_concurrency/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 의존성 추가
`Cargo.toml`에 `rayon` 라이브러리가 추가되어 있는지 확인합니다.

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
use rayon::prelude::*;

// 병렬 합계 계산
fn parallel_sum(numbers: Vec<i32>) -> i32 {
    numbers.par_iter().sum()
}

// 병렬 제곱 계산
fn parallel_map_square(numbers: Vec<i32>) -> Vec<i32> {
    numbers.par_iter().map(|&x| x * x).collect()
}

fn main() {
    println!("--- 1. Parallel Iteration with Rayon ---");
    let numbers: Vec<i32> = (1..1_000_000).collect();

    let sum = parallel_sum(numbers.clone());
    println!("Parallel Sum: {}", sum);

    let squares = parallel_map_square(numbers);
    println!("First 5 squares: {:?}", &squares[..5]);
}
```

## 4. 실행 및 테스트
```bash
cargo run
cargo test
```

**실습 포인트:**
1. **`par_iter()` vs `iter()`:** `par_iter()`를 사용했을 때와 일반 `iter()`를 사용했을 때의 성능 차이를 큰 데이터셋에서 관찰해 보세요.
2. **Work Stealing:** Rayon이 어떻게 CPU 코어를 효율적으로 사용하는지 이론적으로 학습해 보세요.
3. **Overhead:** 아주 작은 데이터셋(예: 10개의 요소)에서 병렬 처리를 시도했을 때 왜 성능이 더 낮게 나올 수 있는지 생각해보세요.

## 5. 도전 과제 (Extra Credit)
- `rayon`의 `reduce` 또는 `fold` 메서드를 사용하여 병렬로 값을 누적하는 코드를 작성해 보세요.
- `rayon::join` 함수를 사용하여 두 개의 서로 다른 작업을 동시에 실행하는 예제를 만들어 보세요.
- `criterion` 라이브러리를 사용하여 일반 반복문과 `Rayon` 병렬 반복문의 성능을 벤치마킹해 보세요.
