# Day 7 실습 가이드: 스마트 포인터 - Rc<T> (Reference Counting)

이 가이드는 여러 소유자가 하나의 데이터를 공유해야 하는 상황을 해결하기 위한 스마트 포인터인 `Rc<T>`(Reference Counted)의 개념과 사용법을 배우는 것을 목표로 합니다.

## 1. 목표
- `Rc<T>`의 동작 원리(참조 카운팅)를 이해한다.
- 여러 변수가 동일한 힙 데이터를 공유하는 방법을 익힌다.
- `Rc<T>`의 한계(불변성)를 확인한다.

## 2. 프로젝트 구조
```text
day7_rc_sharing/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day7_rc_sharing
cd day7_rc_sharing
```

### 단계 2: Rc 공유 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 참조 카운트가 어떻게 변하는지 관찰하세요.

```rust
use std::rc::Rc;

fn main() {
    println!("--- Rc<T> Reference Counting 실습 ---");

    // 1. 데이터 생성 및 첫 번째 소유자
    let shared_data = Rc::new(String::from("공유 데이터"));
    println!("초기 참조 카운트: {}", Rc::strong_count(&shared_data));

    // 2. 새로운 소유자 추가 (Clone)
    let owner2 = Rc::clone(&shared_data);
    println!("owner2 추가 후 카운트: {}", Rc::strong_count(&shared_data));

    let owner3 = Rc::clone(&shared_data);
    println!("owner3 추가 후 카운트: {}", Rc::strong_count(&shared_data));

    // 3. 데이터 접근
    println!("shared_data 내용: {}", shared_data);
    println!("owner2 내용: {}", owner2);
    println!("owner3 내용: {}", owner3);

    // 4. 소유자 해제 (Scope 종료 시뮬레이션)
    println!("\n--- 소유자 일부 해제 ---");
    drop(owner2);
    println!("owner2 해제 후 카운트: {}", Rc::strong_count(&shared_data));

    drop(owner3);
    println!("owner3 해제 후 카운트: {}", Rc::strong_count(&shared_data));

    // 5. Rc의 한계: 불변성(Immutability)
    println!("\n--- Rc의 한계: 불변성 확인 ---");
    // let mut_ref = Rc::clone(&shared_data);
    // mut_ref.push_str(" 수정 시도"); // [실습] 이 주석을 해제해보세요. 
    // 에러 발생 이유: Rc<T>는 내부 데이터를 수정할 수 있는 권한을 주지 않습니다.
    // 데이터를 수정하려면 Rc<RefCell<T>>와 같은 조합이 필요합니다.
    
    println!("현재 데이터: {}", shared_data);
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**확인 사항:**
- `Rc::strong_count`를 통해 참조 카운트가 `clone()`할 때마다 증가하고, `drop()`할 때마다 감소하는지 확인하세요.
- `Rc<T>`를 통해 공유된 데이터가 모두 동일한 값을 가리키고 있는지 확인하세요.
- `Rc<T>` 내부의 값을 직접 수정하려고 할 때 발생하는 컴파일 에러를 확인하세요.

## 5. 도전 과제 (Extra Credit)
- `Rc<RefCell<T>>`를 사용하여 `Rc`로 공유하면서도 내부 값을 수정할 수 있는 구조를 만들어 보세요.
- `Rc::try_unwrap`을 사용하여 참조 카운트가 1일 때 소유권을 다시 가져오는 실험을 해보세요.
