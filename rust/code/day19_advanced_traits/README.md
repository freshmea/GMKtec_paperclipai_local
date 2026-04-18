# Day 19. Advanced Traits & Generics 실습 가이드

이 프로젝트는 Rust의 핵심 추상화 도구인 Traits와 Generics를 심화 학습하기 위한 실습 공간입니다.

## 실습 목표
1. **Trait Bounds**를 사용하여 제네릭의 범위를 제한하는 방법을 익힙니다.
2. **Associated Types**를 사용하여 Trait의 유연성을 높이는 방법을 배웁니다.
3. **Static Dispatch**와 **Dynamic Dispatch**의 차이점과 성능 영향을 이해합니다.

## 실습 내용

### 1. Trait 정의 및 구현
`src/main.rs`에서 `Summary` 트레이트를 정의하고, 이를 다양한 타입(`String`, `i32`, `CustomStruct`)에 대해 구현합니다.

### 2. 제네릭 함수 작성
`Summary` 트레이트를 구현한 타입들을 인자로 받아 처리하는 제네릭 함수 `print_summary<T: Summary>(item: T)`를 작성합니다.

### 3. 다형성(Polymorphism) 구현
`Box<dyn Summary>`를 사용하여 런타임에 서로 다른 타입들을 하나의 컬렉션(`Vec`)에 담아 처리하는 Dynamic Dispatch를 구현합니다.

## 실행 방법

### 1. 프로젝트 실행
```bash
cargo run
```

### 2. 테스트 실행 (필요 시)
```bash
cargo test
```

## 학습 포인트
- `T: Summary` (Trait Bound) vs `Box<dyn Summary>` (Trait Object)의 차이
- 컴파일 타임에 결정되는 것과 런타임에 결정되는 것의 차이
- 왜 `dyn`을 사용할 때 `Box`나 `&`와 같은 포인터가 필요한가?
