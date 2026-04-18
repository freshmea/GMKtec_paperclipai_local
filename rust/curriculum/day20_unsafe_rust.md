# Day 20: Unsafe Rust - Rust의 한계와 제어권

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 안전성 보장(Safety Guarantees)을 우회해야 하는 특수한 상황을 이해하고, `unsafe` 키워드를 사용하여 메모리에 직접 접근하거나 외부 라이브러리(FFI)와 상호작용하는 방법을 배우는 것입니다.

* `unsafe` 블록의 의미와 역할 이해
* Raw Pointers (`*const T`, `*mut T`) 사용법 숙지
* `unsafe`를 사용해야 하는 5가지 상황 학습
* FFI (Foreign Function Interface)의 기초 개념 파악
* `unsafe` 코드 작성 시의 책임과 안전한 추상화(Safe Abstraction) 설계 능력 배양

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:30 | **Introduction to Unsafe** | Safe Rust의 한계, `unsafe`의 정의, 왜 필요한가? |
| 00:30 - 01:15 | **Raw Pointers & Dereferencing** | `*const T` vs `*mut T`, Reference vs Pointer, Dereferencing |
| 01:15 - 02:00 | **The 5 Unsafe Superpowers** | Deref raw pointers, Call unsafe functions, Access mutable static, Access/modify fields of `union`, Implement unsafe traits |
| 02:00 - 02:45 | **FFI & C Interoperability** | `extern "C"`, C 함수 호출, `libc` crate 활용 |
| 02:45 - 03:30 | **Safe Abstractions** | `unsafe`를 감싸는 Safe Wrapper 설계, `UnsafeCell` 개념 소개 |
| 03:30 - 04:00 | **Lab & Assignment** | 실습: Raw Pointer를 이용한 Linked List 구현 또는 FFI 호출 |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Safe Rust vs Unsafe Rust
Rust의 컴파일러는 메모리 안전성을 보장하지만, 하드웨어를 직접 제어하거나 성능 극대화를 위해 메모리 레이아웃을 직접 다뤄야 할 때 제약이 발생합니다. `unsafe`는 컴파일러에게 "이 부분은 내가 책임질 테니, 타입 체크는 하되 메모리 접근 규칙 검사는 생략해라"라고 명령하는 것입니다.

### 3.2. Raw Pointers
* `&T` (Reference): 항상 유효하며, null이 아니며, Aliasing 규칙을 따릅니다.
* `*const T` / `*mut T` (Raw Pointer): null일 수 있고, 유효하지 않은 메모리를 가리킬 수 있으며, Aliasing 규칙을 무시합니다. **Dereferencing(역참조)은 반드시 `unsafe` 블록 내에서 이루어져야 합니다.**

### 3.3. Unsafe Superpowers
1. **Dereference raw pointers**: `*ptr` 연산.
2. **Call unsafe functions**: `unsafe fn` 호출.
3. **Access/modify mutable static variables**: 전역 가변 변수 접근.
4. **Access fields of a `union`**: Union 타입의 필드 접근.
5. **Implement unsafe traits**: 구현 시 안전성을 보장해야 하는 trait 구현.

### 3.4. Safe Abstraction (중요)
`unsafe` 코드는 그 자체로 존재해서는 안 됩니다. `unsafe`를 사용하여 구현하되, 외부 사용자는 안전하게 사용할 수 있도록 **Safe Wrapper**를 제공하는 것이 Rust 프로그래밍의 핵심입니다.

## 4. 데모 포인트 (Demonstration Points)

1. **Segmentation Fault 유도:** 유효하지 않은 메모리 주소를 Raw Pointer로 가리킨 뒤 dereference 하여 프로그램이 죽는 모습 확인.
2. **Mutable Static 활용:** `static mut` 변수를 수정하며 발생하는 Race Condition 위험성 보여주기.
3. **FFI Demo:** C의 `printf` 함수를 Rust에서 호출하여 외부 라이브러리와의 연결 확인.
4. **Safe Wrapper 구현:** `unsafe`한 Raw Pointer 기반의 데이터 구조를 `struct`와 `impl`로 감싸서 안전하게 노출하기.

## 5. 실습 과제 (Lab/Assignment)

### 과제: Raw Pointer를 이용한 간단한 Stack 구현하기

**요구사항:**
1. `UnsafeStack<T>` 구조체를 만드세요.
2. 내부 데이터는 `Box<[T]>` 또는 Raw Pointer를 사용하여 관리하세요.
3. `push`, `pop` 메서드를 구현하되, 내부적으로는 `unsafe` 블록을 사용하여 포인터 연산을 수행하세요.
4. **핵심:** 외부 사용자가 이 Stack을 사용할 때는 `unsafe` 키워드를 쓰지 않아도 되도록 완벽하게 **Safe Abstraction**을 제공해야 합니다.
5. `pop` 시점에 스택이 비어있다면 `Option<T>`를 반환하도록 설계하세요.

**검증 방법:**
- `cargo test`를 통해 다양한 시나리오(빈 스택에서 pop, 여러 번 push/pop 등)를 테스트하세요.
- `cargo clippy`를 실행하여 잠재적인 위험 요소가 있는지 확인하세요.
