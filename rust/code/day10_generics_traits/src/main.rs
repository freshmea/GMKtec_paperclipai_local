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
    fn importance(&self) -> &'static str; // Added for Extra Credit
    
    // Default Implementation
    fn print_summary(&self) {
        println!("Summary: {} [Importance: {}]", self.summarize(), self.importance());
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
    fn importance(&self) -> &'static str {
        "High"
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
    fn importance(&self) -> &'static str {
        "Low"
    }
}

// --- 3. Trait Bounds: 제약 조건이 있는 Generics ---
// T는 반드시 Summary 트레이트를 구현해야 함을 명시
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 'where' 절을 사용한 가독성 높은 제약 조건
// Added Summary constraint for Extra Credit
fn print_anything<T>(item: T) 
where 
    T: Display + std::fmt::Debug + Summary 
{
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
    println!("Summary: {}", item.summarize());
}

// --- 4. Trait Objects: 런타임 다형성 (dyn Trait) ---
fn print_summaries(items: Vec<Box<dyn Summary>>) {
    for item in items {
        item.print_summary();
    }
}

// Wrapper for Extra Credit: Implementing Summary for types that are Display + Debug
struct WrappedType<T>(T);

impl<T> std::fmt::Debug for WrappedType<T> where T: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> std::fmt::Display for WrappedType<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Summary for WrappedType<T> where T: Display + std::fmt::Debug {
    fn summarize(&self) -> String {
        format!("{}", self.0)
    }
    fn importance(&self) -> &'static str {
        "N/A"
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
    
    // Using WrappedType to satisfy the new Summary constraint in print_anything
    print_anything(WrappedType(123));
    print_anything(WrappedType("Generic String"));

    println!("\n--- 4. Trait Objects (dyn Summary) ---");
    let list: Vec<Box<dyn Summary>> = vec![
        Box::new(article),
        Box::new(tweet),
    ];
    print_summaries(list);
}
