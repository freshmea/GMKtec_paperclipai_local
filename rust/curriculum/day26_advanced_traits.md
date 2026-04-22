# Day 26: Advanced Traits - Associated Types and Trait Bounds

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 트레이트(Trait) 시스템을 한 단계 더 깊게 파고들어, 제네릭 프로그래밍의 유연성을 극대화하는 고급 기법들을 학습하는 것입니다.

* **Associated Types (연관 타입)**의 개념과 사용 이유 이해
* **Trait Bounds (트레이트 경계)**의 복합적인 활용법 숙지
* **Generic vs Associated Types**의 차이점 및 선택 기준 학습
* **Higher-Kinded Traifs (HKT)의 유사 구현** 및 복잡한 제약 조건 설계

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:45 | **Review: Basic Traits & Generics** | 기존 Generic과 Trait Bound 복습 |
| 00:45 - 01:45 | **Associated Types** | `type Item;`을 이용한 타입 정의, 왜 Generic보다 유리한가? |
| 01:45 - 02:45 | **Complex Trait Bounds** | `where` 절을 이용한 복잡한 제약 조건, `for<'a> Trait<'a>` (HRTB) 맛보기 |
| 02:45 - 03:30 | **Trait Objects vs Generics** | 정적 디스패치(Static Dispatch) vs 동적 디스패치(Dynamic Dispatch) |
| 03:30 - 04:00 | **Lab & Assignment** | 실습: 연관 타입을 사용한 Iterator 패턴 구현 또는 데이터 저장소 추상화 |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Associated Types (연관 타입)
제네릭 타입 파라미터는 호출할 때마다 타입을 지정해야 하지만, 연관 타입은 트레이트를 구현하는 타입에 의해 결정됩니다.
* **Generic:** `trait Container<T>` $\rightarrow$ `impl Container<i32> for MyBox`, `impl Container<String> for MyBox` (한 타입이 여러 번 구현 가능)
* **Associated Type:** `trait Container { type Item; }` $\rightarrow$ `impl Container for MyBox { type Item = i32; }` (한 타입에 대해 하나의 구현만 가능)

### 3.2. Trait Bounds (트레이트 경계)
제네릭 함수나 구조체가 특정 기능을 수행할 수 있음을 보장하기 위해 사용합니다.
* `where` 절을 사용하면 코드가 훨씬 읽기 쉬워집니다.
* `T: Clone + Debug + Send`와 같이 여러 조건을 결합할 수 있습니다.

### 3.3. Static vs Dynamic Dispatch
* **Static (Generics):** 컴파일 타임에 구체적인 타입이 결정되어 인라인화(Inlining)가 가능하므로 매우 빠릅니다. (Monomorphization)
* **Dynamic (`dyn Trait`):** 런타임에 가상 함수 테이블(vtable)을 통해 결정되므로 유연하지만 약간의 오버헤드가 있습니다.

## 4. 데모 포인트 (Demonstration Points)

1. **Generic의 한계:** 하나의 구조체가 동일한 트레이트에 대해 여러 타입을 가져야 할 때의 복잡성 보여주기.
2. **Associated Type의 깔끔함:** `Iterator` 트레이트의 `type Item`을 통해 어떻게 타입이 결정되는지 분석.
3. **Where 절의 마법:** 복잡한 제약 조건을 `where` 절로 정리하여 가독성을 높이는 전/후 비교.

## 5. 실습 과제 (Lab/Assignment)

### 과제: 연관 타입을 사용한 Repository 패턴 구현

**요구사항:**
1. `Repository` 트레이트를 만드세요.
2. 이 트레이트는 `type Entity;`와 `type Id;`라는 두 개의 연관 타입을 가져야 합니다.
3. 트레이트 메서드로 `get_by_id(&self, id: Self::Id) -> Option<Self::Entity>`를 정의하세요.
4. `User` 구조체와 `UserId` 타입을 만드세요.
5. `UserRepository` 구조체를 만들어 `Repository` 트레이트를 구현하세요.
6. **핵점:** `Repository`를 구현할 때 `Self::Entity`와 `Self::Id`를 명시적으로 지정하여 타입 안전성을 확보하세요.

**검증 방법:**
- `cargo test`를 통해 실제 데이터가 `Id`를 통해 올바르게 조회되는지 확인하세요.
- `cargo clippy`를 통해 트레이트 설계의 적절성을 확인하세요.
