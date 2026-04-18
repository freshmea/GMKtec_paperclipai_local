# Day 12: Smart Pointers (Box, Rc, RefCell)

## 1. 학습 목표 (Objectives)
- Rust의 `Smart Pointer` 개념을 이해하고, 일반 참조(`&T`)와의 차이점을 파악합니다.
- `Box<T>`를 사용하여 힙(Heap) 메모리 할당을 제어하는 방법을 배웁니다.
- `Rc<T>`를 사용하여 단일 소유권 모델을 넘어선 참조 횟수 기반 공유(Reference Counting)를 이해합니다.
- `RefCell<T>`를 통해 내부 가변성(Interior Mutability) 패턴을 구현하는 방법을 익힙니다.

## 2. 세션 일정 (Session Breakdown)

| 시간 (Time) | 주제 (Topic) | 내용 (Details) |
| :--- | :--- | :--- |
| 00:00 - 00:30 | Smart Pointer 개요 | Stack vs Heap, Pointer의 개념, `Deref`와 `Drop` 트레이트 |
| 00:30 - 01:30 | `Box<T>` | Heap 할당, 재귀적 타입(Recursive Types), 소유권 이전 |
| 01:30 - 02:30 | `Rc<T>` | Reference Counting, 다중 소유권, `clone()`의 동작 원리 |
| 02:30 - 03:30 | `RefCell<T>` | Interior Mutability, 런타임 Borrow Checking, `borrow()` vs `borrow_mut()` |
| 03:30 - 04:00 | 종합 실습 및 Q&A | 실습 코드 실행 및 문제 해결 |

## 3. 핵심 개념 (Key Concepts)

### 3.1 Smart Pointers란?
일반적인 참조(`&T`)는 단순히 메모리 주소를 가리키지만, Smart Pointer는 데이터와 함께 **추가적인 메타데이터**나 **특수한 동작(Logic)**을 포함하는 구조체입니다.
- **Deref 트레이트:** 포인터를 역참조하여 내부 데이터에 접근할 수 있게 합니다.
- **Drop 트레이트:** 변수의 스코프가 끝날 때 메모리 해제 등 정리 작업을 수행합니다.

### 3.2 Box<T> (Unique Ownership on the Heap)
데이터를 스택이 아닌 힙에 할당합니다.
- **용도:** 
  - 큰 데이터를 스택에서 옮길 때 비용 절감.
  - 컴파일 타임에 크기를 알 수 없는 재귀적 데이터 구조(예: Linked List, Tree) 정의.
- **특징:** 단일 소유권(Single Ownership)을 유지합니다.

### 3.3 Rc<T> (Reference Counting)
`Rc`는 "Reference Counted"의 약자로, 하나의 데이터를 여러 곳에서 공유할 때 사용합니다.
- **용도:** 데이터의 소유권이 명확하지 않고 여러 곳에서 읽기 전용으로 참조해야 할 때.
- **특징:** 
  - 힙에 참조 횟수를 기록합니다.
  - `clone()` 호출 시 데이터가 복사되는 것이 아니라, 참조 횟수만 증가합니다.
  - **주의:** `Rc`는 읽기 전용(Immutable) 공유만 가능합니다.

### 3.4 RefCell<T> (Interior Mutability)
Borrow Checker의 규칙(한 번에 하나의 가변 참조 또는 여러 개의 불변 참조)을 **런타임**에 검사합니다.
- **용도:** 불변 참조(`&T`)를 통해서도 내부 데이터를 수정해야 할 때.
- **특징:** 
  - 컴파일 타임이 아닌 런타임에 Borrow 규칙을 확인합니다.
  - 규칙 위반 시(예: 가변 참조가 있는 상태에서 또 다른 가변 참조 시도) `panic!`이 발생합니다.

## 4. 데모 포인트 (Demonstration Points)
1. `Box<T>`를 사용하여 재귀적 `Node` 구조체 만들기.
2. `Rc<T>`를 사용하여 여러 노드가 하나의 데이터를 가리키는 그래프 구조 시뮬레이션.
3. `Rc<RefCell<T>>` 조합을 사용하여 공유 가능한 가변 데이터 구조 만들기.

## 5. 실습 과제 (Lab/Assignment)

### 과제 1: 재귀적 트리 구조 (Box)
`Box<T>`를 사용하여 간단한 이진 트리(Binary Tree) 구조체를 구현하세요. 각 노드는 값과 왼쪽/오른쪽 자식 노드를 가집니다.

### 과제 2: 공유되는 설정 데이터 (Rc)
애플리케이션의 `Config` 구조체를 생성하고, `Rc<Config>`를 사용하여 여러 서비스 모듈이 동일한 설정값을 읽을 수 있도록 구현하세요.

### 과제 3: 공유 가능한 카운터 (Rc + RefCell)
여러 곳에서 공유하면서 동시에 값을 수정할 수 있는 `SharedCounter`를 구현하세요. `Rc<RefCell<u32>>` 패턴을 사용해야 합니다.
