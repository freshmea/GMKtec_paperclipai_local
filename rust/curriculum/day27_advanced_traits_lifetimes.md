# Day 27: Advanced Traits - Lifetimes, Higher-Rank Trait Bounds (HRTB), and Object Safety

## 1. 학습 목표 (Objectives)

본 세션의 목표는 트레이트 시스템의 가장 난해한 부분 중 하나인 라이프타임(Lifetimes)과 트레이트 경계(Trait Bounds)를 깊이 있게 이해하고, 동적 디스패치(Dynamic Dispatch)를 위한 객체 안전성(Object Safety)을 학습하는 것입니다.

* 트레이트와 라이프타임의 결합 이해
* Higher-Rank Trait Bounds (HRTB)의 개념 및 필요성 파악
* 트레이트 객체(Trait Objects)를 위한 Object Safety 조건 이해
* `dyn Trait` 사용 시 발생하는 제약 사항 및 해결 방법 학습

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:45 | **Lifetimes in Traits** | 트레이트 정의 시 라이프타임 매개변수 사용, `self`의 라이프타임 제약 |
| 00:45 - 01:45 | **Higher-Rank Trait Bounds (HRTB)** | `for<'a>` 문법, 클로저와 라이프타임의 관계, Generic vs HRTB |
| 01:45 - 02:45 | **Object Safety (Part 1)** | `dyn Trait`의 원리, 왜 모든 트레이트를 객체로 만들 수 없는가? |
| 02:45 - 03:30 | **Object Safety (Part 2)** | `Self` 타입 사용 제한, Generic 메서드 제한, `Sized` 제약 |
| 03:30 - 04:00 | **Lab & Assignment** | 실습: Object Safety를 준수하는 트레이트 설계 및 `dyn` 활용 |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Lifetimes in Traits
트레이트가 참조를 다룰 때, 그 참조가 얼마나 유효해야 하는지를 트레이트 정의 단계에서 명시해야 합니다.
* `trait MyTrait<'a> { ... }`: 트레이트 자체가 특정 라이프타임에 종속됨.
* `trait MyTrait { fn foo(&'a self); }`: 트레이트 메서드가 특정 라이프타임의 참조를 요구함.

### 3.2. Higher-Rank Trait Bounds (HRTB)
HRTB는 "모든 가능한 라이프타임에 대해" 트레이트가 성립해야 함을 의미합니다.
* **Syntax:** `for<'a> F: Fn(&'a i32)`
* **Why?** 클로저가 인자로 받는 참조자의 라이프타임이 호출 시점에 결정될 때, 그 어떤 라이프타임이 들어와도 안전하게 동작함을 보장하기 위해 필요합니다.

### 3.3. Object Safety
모든 트레이트를 `Box<dyn Trait>`와 같이 트레이트 객체로 만들 수 있는 것은 아닙니다. 컴파일러가 런타임에 함수를 호출하기 위한 vtable을 구성할 수 있어야 하기 때문입니다.

**Object Safety를 위반하는 주요 사례:**
1. 트레이트 메서드가 `Self: Sized`를 요구하거나 `Self`를 반환하는 경우.
2. 트레이트 메서드에 제네릭 파라미터가 있는 경우.
3. 트레이트 메서드가 `Self` 타입의 참조를 인자로 받는 경우 (일부 조건부 허용).

## 4. 데모 포인트 (Demonstration Points)

1. **HRTB의 필요성:** 일반 제네릭 라이프타임과 `for<'a>`의 차이를 클로저 예제를 통해 시연.
2. **Object Safety 위반:** `dyn Trait`를 사용하려 할 때 컴파일 에러가 발생하는 상황(예: 제네릭 메서드 포함) 재현.
3. **해결책 제시:** Object Safety를 유지하면서도 원하는 기능을 구현하는 패턴(예: 메서드를 분리하거나 `Sized` 제약을 조절) 보여주기.

## 5. 실습 과제 (Lab/Assignment)

### 과제: Object Safety를 준수하는 Plugin System 구현

**요구사항:**
1. `Plugin` 트레이트를 만드세요.
2. 이 트레이트는 `name(&self) -> &str` 메서드를 가집니다. (이 메서드는 Object Safety를 만족해야 합니다.)
3. **실패 사례 구현:** `Plugin` 트레이트에 `process<T>(&self, data: T)`와 같은 제네릭 메서드를 추가하고, 이를 `Vec<Box<dyn Plugin>>`에 담으려 할 때 발생하는 컴파일 에러를 확인하세요.
4. **성공 사례 구현:** 제네릭을 제거하거나, 별도의 `Data` 트레이트를 사용하여 Object Safety를 준수하도록 리팩토링하세요.
5. `Vec<Box<dyn Plugin>>`을 순회하며 각 플러그인의 이름을 출력하는 코드를 작성하세요.

**검증 방법:**
- `cargo test`를 통해 플러그인 리스트가 올바르게 순회되는지 확인하세요.
- `cargo clippy`를 통해 설계의 적절성을 확인하세요.
