# Day 6 실습 가이드: 안전한 공유 저장소 (Simple Shared Store)

이 가이드는 소유권과 빌림의 규칙을 활용하여, 데이터를 안전하게 보관하고 읽고 수정할 수 있는 구조체를 설계하는 것을 목표로 합니다.

## 1. 목표
- 구조체(Struct) 정의 및 필드 소유권 이해.
- 메서드(`impl`)에서 불변 참조(`&self`)와 가변 참조(`&mut self`) 활용.
- 빌림 규칙을 위반하는 상황을 코드로 재현하고 컴파일러의 방어를 확인.

## 2. 프로젝트 구조
```text
day6_shared_store/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day6_shared_store
cd day6_shared_store
```

### 단계 2: 저장소 구현 (`src/main.rs`)
`src/main.rs`에 다음 코드를 작성하세요. `DataStore`라는 이름의 구조체를 만들어 데이터를 관리합니다.

```rust
struct DataStore {
    items: Vec<String>,
}

impl DataStore {
    // 새로운 저장소 생성
    fn new() -> Self {
        DataStore {
            items: Vec::new(),
        }
    }

    // 데이터 추가 (가변 참조 필요)
    fn add(&mut self, item: String) {
        self.items.push(item);
    }

    // 데이터 읽기 (불변 참조 필요)
    fn get_item(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    // 데이터 수정 (가변 참조 필요)
    fn update_item(&mut self, index: usize, new_value: String) {
        if let Some(item) = self.items.get_mut(index) {
            *item = new_value;
        }
    }

    // 전체 목록 출력 (불변 참조 필요)
    fn list_all(&self) {
        println!("--- 현재 저장된 목록 ---");
        for (i, item) in self.items.iter().enumerate() {
            println!("{}: {}", i, item);
        }
    }
}

fn main() {
    let mut store = DataStore::new();

    // 1. 데이터 추가
    store.add(String::from("Rust 기초"));
    store.add(String::from("소유권 이해"));

    // 2. 데이터 읽기
    println!("첫 번째 아이템: {:?}", store.get_item(0));

    // 3. 데이터 목록 보기
    store.list_all();

    // 4. 데이터 수정
    store.update_item(0, String::from("Rust 마스터"));
    println!("수정 후 첫 번째 아이템: {:?}", store.get_item(0));

    // 5. [실습] 빌림 규칙 위반 시도
    println!("\n--- 빌림 규칙 위반 테스트 ---");
    
    let item_ref = store.get_item(1); // 불변 참조를 가져옴
    
    // 아래 주석을 해제하여 컴파일 에러를 유도해보세요.
    // store.add(String::from("새로운 항목")); 
    // 에러 이유: item_ref(불변 참조)가 살아있는 동안 store(가변 참조가 필요함)를 수정할 수 없음.

    println!("참조된 아이템: {:?}", item_ref);
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. `item_ref`를 사용하는 코드가 있는 상태에서 `store.add(...)`를 호출하면 어떤 에러가 발생하는지 확인하세요.
2. 에러 메시지에서 "cannot borrow `store` as mutable because it is also borrowed as immutable"라는 문구를 찾아보세요.
3. 에러를 해결하려면 `item_ref`의 사용 범위(Scope)를 어떻게 조절해야 할지 고민해 보세요 (예: `println!` 이후에 `add` 호출하기).

## 5. 도전 과제 (Extra Credit)
- `DataStore`에 특정 인덱스의 아이템을 삭제하는 `remove_item` 메서드를 추가해 보세요.
- `get_item` 메서드가 `Option<&String>` 대신 `Option<String>`을 반환하게 만들려면 어떤 문제가 생길까요? (소유권 관점)
- `DataStore`가 생성될 때 초기 용량(capacity)을 설정할 수 있는 기능을 추가해 보세요.
