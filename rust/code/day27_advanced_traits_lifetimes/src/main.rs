// --- 1. Explicit Lifetimes in Structs ---
// Lifetime은 참조가 유효해야 하는 범위를 컴파일러에게 알려줍니다.
// 구조체가 참조를 포함할 경우, 구조체의 수명은 참조의 수명보다 길 수 없음을 명시해야 합니다.

struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book<'a> {
    fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }

    fn display(&self) {
        println!("Book: {} by {}", self.title, self.author);
    }
}

// --- 2. Lifetime Elision and Explicit Lifetimes in Functions ---
// 컴파일러는 특정 규칙에 따라 라이프타임을 추론(Elision)하지만, 
// 참조가 여러 개 들어오고 반환값의 수명이 불분명할 때는 명시해야 합니다.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// --- 3. Advanced Trait Bounds with Lifetimes ---
// Trait Bound에 Lifetime 제약을 걸어, 특정 수명을 가진 타입만 허용하도록 제한할 수 있습니다.

trait Validator<'a> {
    fn validate(&self, input: &'a str) -> bool;
}

struct RegexValidator {
    pattern: String,
}

impl<'a> Validator<'a> for RegexValidator {
    fn validate(&self, input: &'a str) -> bool {
        // 실제로는 regex crate를 사용하겠지만, 여기서는 단순 비교로 대체
        input.contains(&self.pattern)
    }
}

// Generic function with lifetime and trait bound
fn process_with_validator<'a, T>(data: &'a str, validator: T) 
where 
    T: Validator<'a> 
{
    if validator.validate(data) {
        println!("Data '{}' is valid!", data);
    } else {
        println!("Data '{}' is invalid!", data);
    }
}

fn main() {
    println!("--- 1. Explicit Lifetimes in Structs ---");
    let title = String::from("Rust Programming");
    let author = String::from("Steve Klabnik");
    
    let book = Book::new(&title, &author);
    book.display();

    println!("\n--- 2. Lifetime Elision & Functions ---");
    let s1 = String::from("long string");
    let s2 = "short";
    let result = longest(&s1, s2);
    println!("Longest: {}", result);

    println!("\n--- 3. Advanced Trait Bounds with Lifetimes ---");
    let validator = RegexValidator { pattern: String::from("Rust") };
    process_with_validator("I love Rust", &validator);
    process_with_validator("I love Python", &validator);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_lifetime() {
        let title = String::from("Test");
        let author = String::from("Tester");
        let book = Book::new(&title, &author);
        assert_eq!(book.title, "Test");
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("apple", "pie"), "apple");
    }

    #[test]
    fn test_validator() {
        let v = RegexValidator { pattern: String::from("test") };
        assert!(v.validate("this is a test"));
        assert!(!v.validate("no match"));
    }
}
