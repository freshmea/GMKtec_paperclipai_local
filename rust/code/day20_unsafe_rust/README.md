# Day 20: Unsafe Rust 실습 가이드

이 프로젝트는 `unsafe` 키워드를 사용하여 Rust의 안전성 보장(Safety Guarantees)을 우회하고, Raw Pointer를 직접 다루는 방법을 학습하기 위한 예제입니다.

## 프로젝트 개요

`UnsafeStack<T>`는 내부적으로 `std::alloc`을 통한 수동 메모리 할당과 Raw Pointer 연산을 사용하여 구현된 스택 자료구조입니다. 

이 예제의 핵심 학습 포인트는 다음과 같습니다:
1. **Manual Memory Management**: `std::alloc::alloc`과 `dealloc`을 사용하여 메모리를 직접 할당하고 해제합니다.
2. **Raw Pointer Manipulation**: `*mut T` 포인터를 생성하고, `ptr.add()`를 통해 주소를 계산하며, `ptr::write`와 `ptr::read`를 사용하여 값을 조작합니다.
3. **Safe Abstraction**: `unsafe`를 사용하여 구현하되, 외부 사용자가 `unsafe` 없이도 안전하게 사용할 수 있도록 `push`, `pop`, `len` 등의 인터페이스를 설계합니다.
4. **Resource Cleanup**: `Drop` 트레이트를 구현하여 스택이 범위를 벗어날 때 할당된 메모리와 내부 요소들을 올바르게 정리(drop)합니다.

## 실행 방법

### 1. 데모 실행
프로그램의 동작을 확인하려면 다음 명령을 실행하세요.
```bash
cargo run
```

### 2. 테스트 실행
구현된 로직의 안정성을 검증하기 위해 다음 명령을 실행하세요.
```bash
cargo test
```

## 주의 사항 (Safety Warnings)

이 코드는 학습용이며, 실제 프로덕션 환경에서는 다음과 같은 위험이 있습니다:
- **Memory Leaks**: `Drop` 구현이 잘못되면 메모리 누수가 발생합니다.
- **Undefined Behavior (UB)**: 잘못된 포인터 연산, 이미 해제된 메모리 접근, 또는 Aliasing 규칙 위반은 정의되지 않은 동작을 유발합니다.
- **Panic Safety**: `push`나 `pop` 도중 패닉이 발생할 경우 메모리 상태가 불완전해질 수 있습니다.

## 학습 질문

- `ptr::write`와 `ptr::read`의 차이점은 무엇인가요?
- 왜 `Drop` 트레이트에서 `ptr::drop_in_place`를 호출해야 하나요?
- 만약 `UnsafeStack`에 `T`가 `String`과 같은 Heap 할당 타입을 담고 있다면, `drop` 과정에서 어떤 일이 벌어질까요?
