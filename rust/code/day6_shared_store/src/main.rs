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
