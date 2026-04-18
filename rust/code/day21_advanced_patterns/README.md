# Day 21: Advanced Rust Patterns 실습 가이드

이 프로젝트는 Rust의 강력한 타입 시스템을 활용하여 런타임 에러를 컴파일 타임으로 끌어올리고, 성능 저하 없이 코드를 더욱 안전하고 명확하게 만드는 고급 패턴들을 학습하기 위한 예제입니다.

## 프로젝트 개요

이 예제는 크게 두 가지 핵심 패턴을 다룹니다:
1. **Newtype Pattern**: 단순한 타입을 구조체로 감싸서 도메인 모델의 의미를 부여하고, 서로 다른 타입 간의 혼용을 방지합니다.
2. **Typestate Pattern**: 객체의 상태를 타입(Type)으로 정의하여, 특정 상태에서만 가능한 동작을 컴파일 타임에 강제합니다.

## 학습 포인트

### 1. Newtype Pattern
- `UserId(u32)`와 `OrderId(u32)`는 모두 내부적으로 `u32`를 가지지만, Rust 컴파일러는 이 둘을 완전히 다른 타입으로 취급합니다.
- 이를 통해 `fn process_user(id: UserId)` 함수에 실수로 `OrderId`를 전달하는 버그를 원천 차단할 수 있습니다.

### 2. Typestate Pattern
- `Order<Created>`, `Order<Paid>`, `Order<Shipped>`와 같이 상태를 타입으로 표현합니다.
- `pay()` 메서드는 `Order<Created>`만 가질 수 있고, `ship()` 메서드는 `Order<Paid>`만 가질 수 있도록 설계합니다.
- 결과적으로 "결제되지 않은 주문을 배송"하는 것과 같은 논리적 오류가 발생하면 컴파일 단계에서 에러가 발생합니다.

### 3. PhantomData
- `Order<State>` 구조체는 실제 필드에 `State`를 저장하지 않지만, 컴파일러에게 `State` 타입 정보가 논리적으로 연결되어 있음을 알리기 위해 `std::marker::PhantomData`를 사용합니다.

## 실행 방법

### 1. 데모 실행
프로그램의 동작을 확인하려면 다음 명령을 실행하세요.
```bash
cargo run
```

### 2. 테스트 실행
구현된 패턴의 안정성을 검증하려면 다음 명령을 실행하세요.
```bash
cargo test
```

## 연습 문제

- Newtype 패턴을 확장하여 `Email(String)` 타입을 만들고, 유효한 이메일 형식인지 검증하는 `Email::parse()` 함수를 구현해 보세요.
- Typestate 패턴을 확장하여 `Order` 상태에 `Cancelled` 상태를 추가하고, 취소된 주문은 더 이상 `pay`나 `ship`을 할 수 없도록 설계해 보세요.
- `PhantomData`가 왜 필요한지, 만약 없다면 컴파일러가 어떤 에러를 내뱉는지 직접 확인해 보세요.
