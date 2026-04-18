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
