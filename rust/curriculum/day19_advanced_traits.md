# Day 19: Advanced Traits & Generics

## 학습 목표 (Objectives)
본 세션에서는 Rust의 강력한 추상화 도구인 Traits와 Generics를 심도 있게 다룹니다. 단순한 인터페이스 구현을 넘어, 코드의 재사용성을 극대화하고 복잡한 시스템을 설계할 수 있는 능력을 배양합니다.

1. **Trait Bounds**를 통한 제네릭 제약 조건 설정 이해
2. **Associated Types**와 **Generic Associated Types (GATs)**의 차이 및 활용법 습득
3. **Blanket Implementations**를 이용한 코드 패턴 최적화
4. **Trait Objects (`dyn Trait`)**와 **Static Dispatch**의 성능 및 유연성 트레이드오프 분석

## 세션 타임라인 (Session Breakdown)
- **09:00 - 10:30**: Generics와 Trait Bounds (상세 제약 조건 및 `where` 절)
- **10:30 - 12:00**: Associated Types vs Generics (언제 무엇을 사용할 것인가?)
- **13:00 - 14:30**: Blanket Implementations 및 Trait Supertraits
- **14:30 - 16:00**: Dynamic Dispatch (`dyn`) vs Static Dispatch (Vtable과 성능)
- **16:00 - 18:00**: 종합 실습: 추상화된 데이터 처리 파이프라인 설계

## 핵심 개념 (Key Concepts)

### 1. Trait Bounds & `where` Clauses
제네릭 함수나 구조체가 특정 기능을 수행할 수 있음을 보장합니다. `where` 절을 사용하여 복잡한 제약 조건을 가독성 있게 관리합니다.

### 2. Associated Types
Trait 내부에 정의된 타입을 통해, 구현체마다 특정 타입을 결정할 수 있게 합니다. Generic Associated Types (GATs)는 라이프타임을 포함한 더욱 강력한 추상화를 가능하게 합니다.

### 3. Static vs Dynamic Dispatch
- **Static Dispatch (`impl Trait`):** 컴파일 타임에 구체적인 타입을 결정 (Monomorphization). 성능상 이점이 크지만 코드 크기가 증가할 수 있습니다.
- **Dynamic Dispatch (`dyn Trait`):** 런타임에 Vtable을 참조하여 호출. 유연성이 높지만 약간의 오버헤드가 발생합니다.

## 데모 포인트 (Demonstration Points)
- 동일한 기능을 수행하는 두 가지 방식(Static vs Dynamic)의 벤치마크 비교
- `Iterator` 트레이트의 내부 구현을 통한 Associated Types의 실제 활용 사례 제시
- `where` 절을 사용한 복잡한 Trait Bound의 가독성 개선 전후 비교

## 실습 과제 (Lab/Assignment)

### 과제 1: Generic Container 설계
제공된 `day19_advanced_traits` 프로젝트에서 다음을 구현하세요.
1. `Summary` 트레이트를 정의합니다.
2. `Summary`를 구현하는 다양한 타입(String, i32, Custom Struct)을 만듭니다.
3. `Summary`를 구현하는 모든 타입에 대해 특정 동작을 수행하는 제네릭 함수 `print_summary<T: Summary>(item: T)`를 작성합니다.

### 과제 2: Trait Object를 이용한 다형성 구현
1. `Summary` 트레이트를 사용하는 `Vec<Box<dyn Summary>>`를 생성합니다.
2. 루프를 돌며 각 객체의 `summarize()` 메서드를 호출하여 다형성을 확인합니다.

## 정리 (Summary)
적절한 추상화는 코드의 유연성을 높이지만, 과도한 추상화는 가독성과 성능을 해칠 수 있습니다. 상황에 맞는 Trait 설계와 Dispatch 전략을 선택하는 것이 Rust 전문가의 역량입니다.
