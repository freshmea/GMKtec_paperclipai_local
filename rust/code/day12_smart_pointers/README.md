# Day 12 실습 가이드: Smart Pointers (Box, Rc, RefCell)

이 가이드는 Rust의 핵심 추상화 도구인 Smart Pointer를 사용하여 메모리 관리 방식을 제어하고, 단일 소유권 모델을 넘어선 데이터 공유 및 내부 가변성 패턴을 배우는 것을 목표로 합니다.

## 1. 목표
- `Box<T>`를 사용하여 데이터를 힙(Heap)에 할당하고 재귀적 구조를 만드는 법을 배운다.
- `Rc<T>`를 사용하여 하나의 데이터를 여러 곳에서 안전하게 공유하는 법을 배운다.
- `RefCell<T>`를 사용하여 불변 참조를 통해서도 내부 데이터를 수정하는 "Interior Mutability" 패턴을 익힌다.
- `Rc<RefCell<T>>` 조합을 통해 공유 가능한 가변 데이터를 다루는 실전 패턴을 경험한다.

## 2. 프로젝트 구조
```text
day12_smart_pointers/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
(이미 생성되어 있습니다.)

### 단계 2: 실습 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라가며 실험해 보세요.

```rust
// --- 1. Box<T>: 힙(Heap) 할당과 재귀적 타입 ---
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    fn append(&mut self, new_node: Node) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(new_node));
    }
}

// --- 2. Rc<T>: 참조 횟수 기반 공유 (Reference Counting) ---
use std::rc::Rc;

#[derive(Debug)]
struct Config {
    api_key: String,
    timeout: u32,
}

// --- 3. RefCell<T>: 내부 가변성 (Interior Mutability) ---
use std::cell::RefCell;

#[derive(Debug)]
struct SharedCounter {
    count: Rc<RefCell<u32>>,
}

impl SharedCounter {
    fn new() -> Self {
        SharedCounter {
            count: Rc::new(RefCell::new(0)),
        }
    }

    fn increment(&self) {
        let mut num = self.count.borrow_mut();
        *num += 1;
    }

    fn get_value(&self) -> u32 {
        *self.count.borrow()
    }
}

fn main() {
    println!("--- 1. Box<T> (Recursive Type) ---");
    let mut head = Node::new(1);
    head.append(Node::new(2));
    head.append(Node::new(3));
    println!("Linked List: {:?}", head);

    println!("\n--- 2. Rc<T> (Shared Ownership) ---");
    let config = Rc::new(Config {
        api_key: String::from("secret_key_123"),
        timeout: 30,
    });

    let service_a_config = Rc::clone(&config);
    let service_b_config = Rc::clone(&config);

    println!("Service A API Key: {}", service_a_config.api_key);
    println!("Service B API Key: {}", service_b_config.api_key);
    println!("Reference count: {}", Rc::strong_count(&config));

    println!("\n--- 3. RefCell<T> (Interior Mutability via Rc) ---");
    let counter_a = SharedCounter::new();
    let counter_b = SharedCounter {
        count: Rc::clone(&counter_a.count),
    };

    counter_a.increment();
    counter_a.increment();
    counter_b.increment();

    println!("Counter A value: {}", counter_a.get_value());
    println!("Counter B value: {}", counter_b.get_value());
    println!("Final shared count: {}", counter_a.get_value());
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **Box:** `Node` 구조체가 왜 `Box<Node>`를 사용하는지, 만약 `Box`가 없다면 왜 컴파일 에러가 발생하는지 생각해보세요 (크기를 알 수 없는 타입 문제).
2. **Rc:** `Rc::clone(&config)`를 호출했을 때 실제 데이터가 복사되는지, 아니면 참조 횟수만 늘어나는지 `strong_count`를 통해 확인하세요.
3. **RefCell:** `increment` 함수 내부에서 `borrow_mut()`를 호출할 때, 왜 `&self` (불변 참조)만 가지고 있음에도 데이터를 수정할 수 있는지 이해해 보세요.

## 5. 도전 과제 (Extra Credit)
- `RefCell`의 런타임 검사 규칙을 위반해 보세요. `counter_a.count.borrow()`로 불변 참조를 가져온 상태에서 `counter_a.count.borrow_mut()`를 시도하여 `panic!`이 발생하는 것을 확인하세요.
- `Rc<RefCell<T>>` 대신 `Arc<Mutex<T>>`를 사용하려면 어떤 라이브러리가 필요하고, 무엇이 달라지는지 조사해 보세요 (멀티스레드 환경에서의 차이).
