# Day 9 실습 가이드: 주문 상태 관리 (Order State Machine)

이 가이드는 Rust의 `enum`을 사용하여 시스템의 상태(State)를 모델링하고, 불가능한 상태 전이를 컴파일 타임에 방지하는 '상태 머신(State Machine)' 패턴을 배우는 것을 목표로 합니다.

## 1. 목표
- `enum`의 각 변체(Variant)에 데이터를 담아 상태 정보를 표현한다.
- `match` 문을 사용하여 상태에 따른 동작을 안전하게 분기한다.
- 특정 상태에서만 가능한 동작을 메서드로 구현하여 비즈니스 로직의 무결성을 보장한다.

## 2. 프로젝트 구조
```text
day9_order_state/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day9_order_state
cd day9_order_state
```

### 단계 2: 상태 머신 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하세요. 주문의 흐름(`Pending` $\rightarrow$ `Paid` $\rightarrow$ `Shipped` $\rightarrow$ `Delivered`)을 구현합니다.

```rust
#[derive(Debug, PartialEq)]
enum OrderStatus {
    Pending,
    Paid { transaction_id: String },
    Shipped { tracking_number: String },
    Delivered,
    Cancelled,
}

struct Order {
    id: u32,
    status: OrderStatus,
}

impl Order {
    fn new(id: u32) -> Self {
        Order {
            id,
            status: OrderStatus::Pending,
        }
    }

    // 결제 처리
    fn pay(&mut self, tx_id: String) -> Result<(), String> {
        match self.status {
            OrderStatus::Pending => {
                self.status = OrderStatus::Paid { transaction_id: tx_id };
                Ok(())
            }
            _ => Err("결제는 Pending 상태에서만 가능합니다.".to_string()),
        }
    }

    // 배송 처리
    fn ship(&mut self, tracking: String) -> Result<(), String> {
        match self.status {
            OrderStatus::Paid { .. } => {
                self.status = OrderStatus::Shipped { tracking_number: tracking };
                Ok(())
            }
            _ => Err("배송은 Paid 상태에서만 가능합니다.".to_string()),
        }
    }

    // 배송 완료 처리
    fn deliver(&mut self) -> Result<(), String> {
        match self.status {
            OrderStatus::Shipped { .. } => {
                self.status = OrderStatus::Delivered;
                Ok(())
            }
            _ => Err("배송 완료는 Shipped 상태에서만 가능합니다.".to_string()),
        }
    }

    // 주문 취소
    fn cancel(&mut self) -> Result<(), String> {
        match self.status {
            OrderStatus::Pending | OrderStatus::Paid { .. } => {
                self.status = OrderStatus::Cancelled;
                Ok(())
            }
            _ => Err("이미 배송 중이거나 완료된 주문은 취소할 수 없습니다.".to_string()),
        }
    }
}

fn main() {
    println!("--- 주문 상태 머신 테스트 ---");

    let mut my_order = Order::new(101);
    println!("초기 상태: {:?}", my_order.status);

    // 1. 정상 흐름: Pending -> Paid -> Shipped -> Delivered
    println!("\n[정상 흐름 테스트]");
    
    if let Err(e) = my_order.pay("TX-999".to_string()) {
        println!("오류: {}", e);
    } else {
        println!("결제 성공! 현재 상태: {:?}", my_order.status);
    }

    if let Err(e) = my_order.ship("TRACK-123".to_string()) {
        println!("오류: {}", e);
    } else {
        println!("배송 시작! 현재 상태: {:?}", my_order.status);
    }

    if let Err(e) = my_order.deliver() {
        println!("오류: {}", e);
    } else {
        println!("배송 완료! 현재 상태: {:?}", my_order.status);
    }

    // 2. 잘못된 흐름 테스트: 이미 완료된 주문을 취소하려고 시도
    println!("\n[잘못된 흐름 테스트 (취소 시도)]");
    match my_order.cancel() {
        Ok(_) => println!("취소 성공"),
        Err(e) => println!("예상된 오류 발생: {}", e),
    }

    // 3. 새로운 주문으로 잘못된 순서 테스트
    println!("\n[잘못된 순서 테스트 (결제 없이 배송)]");
    let mut bad_order = Order::new(102);
    match bad_order.ship("TRACK-ERR".to_string()) {
        Ok(_) => println!("배송 성공"),
        Err(e) => println!("예상된 오류 발생: {}", e),
    }
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **데이터 결합:** `Paid` 상태에는 `transaction_id`가, `Shipped` 상태에는 `tracking_number`가 포함되어 있습니다. 이 데이터들이 상태와 함께 어떻게 묶여 있는지 관찰하세요.
2. **패턴 매칭:** `match self.status`를 통해 현재 상태에 따라 허용되는 동작을 엄격하게 제한하는 로직을 이해합니다.
3. **에러 핸들링:** 잘못된 상태 전이를 시도했을 때 `Result::Err`를 반환하여 시스템의 무결성을 지키는 방법을 확인합니다.

## 5. 도전 과제 (Extra Credit)
- `cancel` 메서드에 `Paid` 상태일 경우 결제 취소 로직(예: 환불 처리)을 추가해 보세요.
- `OrderStatus`에 `Refunded` 상태를 추가하여 흐름을 확장해 보세요.
- `Order` 구조체에 `history: Vec<OrderStatus>` 필드를 추가하여 상태 변화 이력을 기록해 보세요.
