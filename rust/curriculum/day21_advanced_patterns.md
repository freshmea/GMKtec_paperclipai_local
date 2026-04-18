# Day 21: Advanced Rust Patterns - Typestate, Newtype, and Zero-Cost Abstractions

## 1. 학습 목표 (Objectives)

본 세션의 목표는 Rust의 강력한 타입 시스템을 활용하여 런타임 에러를 컴파일 타임으로 끌어올리고, 성능 저하 없이 코드를 더욱 안전하고 명확하게 만드는 고급 패턴들을 학습하는 것입니다.

* **Newtype Pattern**을 통한 도메인 모델의 타입 안전성 강화
* **Typestate Pattern**을 이용한 객체의 상태 전이 및 잘못된 상태 호출 방지
* **Zero-Cost Abstractions**의 개념 이해 및 활용
* **PhantomData**를 활용한 타입 정보의 논리적 결합

## 2. 세션 구성 (Session Breakdown)

| 시간 | 주제 | 주요 내용 |
| :--- | :--- | :--- |
| 00:00 - 00:45 | **Newtype Pattern** | Primitive 타입 래핑, 도메인 의미 부여, 타입 혼용 방지 |
| 00:45 - 01:45 | **Typestate Pattern** | 상태를 타입으로 표현하기, 컴파일 타임 상태 머신 구현 |
| 01:45 - 02:30 | **PhantomData & Marker Traits** | 컴파일러에게 타입 정보 전달하기, 논리적 소유권 표현 |
| 02:30 - 03:30 | **Zero-Cost Abstractions** | Iterator, Closures, Generics가 어떻게 성능을 유지하는가? |
| 03:30 - 04:00 | **Lab & Assignment** | 실습: Typestate를 이용한 안전한 Order/Payment 상태 머신 구현 |

## 3. 핵심 개념 (Key Concepts)

### 3.1. Newtype Pattern
단순한 타입(예: `u32`, `String`)을 구조체로 감싸서 새로운 타입을 만드는 패턴입니다.
* **Why?** `UserId(u32)`와 `OrderId(u32)`를 구분하여, 실수로 ID를 서로 바꿔 넣는 버그를 컴파일 타임에 차단합니다.
* **Cost:** 런타임 오버헤드는 전혀 없습니다 (Zero-cost).

### 3.2. Typestate Pattern
객체의 상태(State)를 내부 필드가 아닌 **타입(Type)**으로 정의하는 패턴입니다.
* **Why?** "결제되지 않은 주문" 상태에서 "배송 시작" 메서드를 호출하는 것과 같은 논리적 오류를 컴파일러가 잡아낼 수 있게 합니다.
* **How?** 상태가 변할 때마다 새로운 타입을 가진 구조체를 반환하여 상태 전이를 명시합니다.

### 3.3. PhantomData
구조체가 특정 타입 `T`를 논리적으로 사용하고 있지만, 실제 필드에는 `T`가 없을 때 사용합니다.
* **Why?** 컴파일러에게 `T`에 대한 소유권이나 생명주기(Lifetime) 정보를 알려주어, 타입 검사 및 드롭(Drop) 체크가 정확히 이루어지도록 돕습니다.

### 3.4. Zero-Cost Abstractions
"사용하지 않는 것에 대해 비용을 지불하지 않는다"와 "사용자가 직접 짠 코드만큼 효율적이다"라는 원칙입니다. Rust의 Generic과 Iterator는 이 원칙을 가장 잘 보여주는 사례입니다.

## 4. 데모 포인트 (Demonstration Points)

1. **Newtype 실습:** `Email(String)`과 `Username(String)`을 만들어 서로 교체 시도 시 컴파일 에러 확인.
2. **Typestate 실습:** `Order<Created>` $\rightarrow$ `Order<Paid>` $\rightarrow$ `Order<Shipped>`로 이어지는 흐름을 구현하고, 중간 단계를 건너뛰는 코드가 컴파일되지 않음을 보여줌.
3. **PhantomData 실습:** 특정 데이터의 소유권을 논리적으로 표시하는 간단한 Wrapper 구현.

## 5. 실습 과제 (Lab/Assignment)

### 과제: Typestate 패턴을 이용한 안전한 HTTP 요청 빌더(Request Builder) 구현

**요구사항:**
1. `HttpRequest` 구조체를 만드세요.
2. 요청은 반드시 `Method` 설정 $\rightarrow$ `URL` 설정 $\rightarrow$ `Body` 설정 순서로 진행되어야 합니다.
3. 각 단계는 서로 다른 타입을 반환해야 합니다 (예: `RequestBuilder<NoMethod>`, `RequestBuilder<MethodSet>`, `RequestBuilder<UrlSet>`).
4. 최종적으로 `build()` 메서드는 모든 필수 정보가 설정된 상태(`RequestBuilder<Ready>`)에서만 호출 가능해야 합니다.
5. **핵심:** 사용자가 순서를 틀리거나 필수 정보를 누락하면 컴파일 에러가 발생해야 합니다.

**검증 방법:**
- `cargo test`를 통해 올바른 빌더 체이닝과 잘못된 체이닝(컴파일 에러 유도)을 확인하세요.
- `cargo clippy`를 실행하여 설계의 깔끔함을 확인하세요.
