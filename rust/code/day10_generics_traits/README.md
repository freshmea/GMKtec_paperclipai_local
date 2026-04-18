# Day 10 실습 가이드: Generics와 Traits

이 가이드는 Rust의 핵심 추상화 도구인 Generics와 Traits를 사용하여 코드의 재사용성을 높이고, 서로 다른 타입들이 공통된 동작을 수행할 수 있도록 설계하는 방법을 배웁니다.

## 1. 목표
- Generics를 사용하여 다양한 타입을 처리하는 범용적인 구조체와 함수를 만든다.
- Traits를 정의하고 구현하여 타입에 공통된 행위를 부여한다.
- Trait Bounds를 사용하여 Generic 타입이 특정 능력을 갖추도록 제한한다.
- Trait Objects (`dyn Trait`)를 사용하여 런타임 다형성을 경험한다.

## 2. 프로젝트 구조
```text
day10_generics_traits/
├── Cargo.toml
└── src/
    └── main.rs
```

## 3. 구현 단계

### 단계 1: 프로젝트 생성
```bash
cargo new day10_generics_traits
cd day10_generics_traits
```

### 단계 2: Generics와 Trait 구현 (`src/main.rs`)
다음 코드를 `src/main.rs`에 작성하고, 각 섹션의 주석을 따라 실험해 보세요.

```rust
use std::fmt::Display;

// --- 1. Generics: 다양한 타입을 담는 구조체 ---
#[derive(Debug)]
struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn get_item(&self) -> &T {
        &self.item
    }
}

// --- 2. Traits: 공통 동작 정의 ---
trait Summary {
    fn summarize(&self) -> String;
    
    // Default Implementation
    fn print_summary(&self) {
        println!("Summary: {}", self.summarize());
    }
}

struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("Headline: {}", self.headline)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// --- 3. Trait Bounds: 제약 조건이 있는 Generics ---
// T는 반드시 Summary 트레이트를 구현해야 함을 명시
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 'where' 절을 사용한 가독성 높은 제약 조건
fn print_anything<T>(item: T) 
where 
    T: Display + std::fmt::Debug 
{
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// --- 4. Trait Objects: 런타임 다형성 (dyn Trait) ---
fn print_summaries(items: Vec<Box<dyn Summary>>) {
    for item in items {
        item.print_summary();
    }
}

fn main() {
    println!("--- 1. Generics ---");
    let int_container = Container::new(10);
    let str_container = Container::new("Hello Rust");
    println!("{:?}", int_container.get_item());
    println!("{:?}", str_container.get_item());

    println!("\n--- 2. Traits & Summary ---");
    let article = NewsArticle {
        headline: String::from("Rust is awesome"),
        content: String::from("It is a safe and fast language."),
    };
    let tweet = Tweet {
        username: String::from("@rust_lang"),
        content: String::from("Hello world!"),
    };

    article.print_summary();
    tweet.print_summary();

    println!("\n--- 3. Trait Bounds ---");
    notify(&article);
    notify(&tweet);
    print_anything(123);
    print_anything("Generic String");

    println!("\n--- 4. Trait Objects (dyn Summary) ---");
    let list: Vec<Box<dyn Summary>> = vec![
        Box::new(article),
        Box::new(tweet),
    ];
    print_summaries(list);
}
```

## 4. 실행 및 테스트
```bash
cargo run
```

**실습 포인트:**
1. **Generics:** `Container<T>`가 어떻게 정수와 문자열을 모두 수용하는지 확인하세요.
2. **Trait Bounds:** `notify` 함수에서 `Summary`를 구현하지 않은 타입을 넣었을 때 발생하는 컴파일 에러를 확인하세요.
3. **Trait Objects:** `Vec<Box<dyn Summary>>`를 통해 서로 다른 타입(`NewsArticle`, `Tweet`)을 하나의 리스트에 담아 처리하는 다형성을 관찰하세요.

## 5. 도전 과제 (Extra Credit)
- `Summary` 트레이트에 `importance()`라는 메서드를 추가하고, `NewsArticle`은 `High`, `Tweet`은 `Low`를 반환하도록 구현해 보세요.
- `print_anything` 함수를 수정하여 `T`가 `Summary`와 `Display`를 동시에 구현해야 하도록 제약을 걸어보세요.
- `dyn Summary` 대신 `&dyn Summary`를 사용하는 함수를 만들어 보세요.
