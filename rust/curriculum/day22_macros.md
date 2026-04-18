# Day 22: Macros in Rust - Declarative and Procedural

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 메타프로그래밍 도구인 매크로(Macros)를 이해하고, 코드 생성을 자동화하여 생산성과 가독성을 높이는 방법을 배우는 것입니다.

* `macro_rules!`를 이용한 선언적 매크로(Declarative Macros) 작성법 숙지
* 매크로 패턴 매칭과 반복자(Repetition) 사용법 학습
* 절차적 매크로(Procedural Macros)의 3가지 종류 이해
* `derive`, `attribute`, `function-like` 매크로의 차이점 및 활용 사례 파악
* 매크로 작성 시의 주의사항과 디버깅 방법

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:45 | **Introduction to Macros** | 매크로 vs 함수, 메타프로그래밍의 필요성, 매크로의 종류 |
| 00:45 - 01:45 | **Declarative Macros (`macro_rules!`)** | 패턴 매칭, 변수 바인딩, 반복자(`$()*`, `$()*`) 사용법 |
| 01:45 - 02:45 | **Procedural Macros Overview** | `proc-macro` crate 설정, TokenStream의 이해 |
| 02:45 - 03:30 | **Derive Macros** | `#[derive(MyTrait)]`를 통한 커스텀 트레이트 자동 구현 |
| 03:30 - 04:00 | **Lab & Assignment** | 실습: 선언적 매크로로 로그 출력기 만들기 또는 간단한 Derive 매크로 작성 |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Macros vs Functions
* **함수:** 런타임에 인자를 전달받아 실행됩니다. 코드의 논리를 실행합니다.
* **매크로:** 컴파일 타임에 코드를 생성(Code Generation)합니다. 코드를 확장(Expansion)합니다.

### 3.2. Declarative Macros (`macro_rules!`)
Rust에서 가장 흔히 쓰이는 매크로 방식으로, 패턴 매칭을 통해 코드를 확장합니다.
* **Syntax:** `macro_rules! name { (pattern) => { expansion } }`
* **Repetition:** `$($x:expr),*`와 같은 문법을 사용하여 여러 인자를 처리할 수 있습니다.

### 3.3. Procedural Macros
함수처럼 입력된 토큰 스트림을 받아 새로운 토큰 스트림을 반환하는 매크로입니다.
1. **Derive macros:** 구조체나 열거형 위에 `#[derive(MyTrait)]`로 붙여 트레이트를 자동 구현합니다.
2. **Attribute macros:** `#[my_attribute]`와 같이 함수나 구조체에 커스텀 속성을 부여합니다.
3. **Function-like macros:** `my_macro!(...)`와 같이 함수처럼 호출하지만, 코드 확장을 수행합니다.

### 3.4. TokenStream
매크로가 다루는 최소 단위인 "토큰"들의 흐름입니다. 매크로는 이 토큰들을 읽고, 변형하고, 새로운 토큰을 만들어 반환합니다.

## 4. 데모 포인트 (Demonstration Points)

1. **`vec!` 매크로 분석:** `vec![1, 2, 3]`이 어떻게 내부적으로 `Vec::new()`와 `push()`로 확장되는지 보여주기.
2. **반복자 매크로:** 여러 개의 인자를 받아 한 번에 처리하는 매크로 작성.
3. **Custom Derive:** `Debug` 트레이트를 직접 구현하는 것과 `derive`를 사용하는 것의 차이 확인.

## 5. 실습 과제 (Lab/Assignment)

### 과제: 선언적 매크로를 이용한 `debug_print!` 매크로 구현

**요구사항:**
1. `debug_print!` 매크로를 만드세요.
2. 이 매크로는 전달받은 모든 인자의 값을 출력해야 합니다.
3. 각 인자의 값 앞에 변수 이름(또는 식의 형태)이 함께 출력되도록 하세요. (예: `debug_print!(a, b + c);` 호출 시 `a = 1, b + c = 5` 출력)
4. 인자의 개수가 가변적이어야 합니다.

**검증 방법:**
- `cargo test`를 통해 다양한 타입과 개수의 인자가 정상적으로 출력되는지 확인하세요.
- `cargo expand` (설치되어 있다면)를 사용하여 매크로가 어떻게 확장되는지 관찰하세요.
