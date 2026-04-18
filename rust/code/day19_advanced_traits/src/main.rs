trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {} by {}", self.title, self.author)
    }
}

impl Summary for String {
    fn summarize(&self) -> String {
        format!("String: {}", self)
    }
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("Number: {}", self)
    }
}

// Task 1: Static Dispatch (Generic with Trait Bound)
fn print_summary_static<T: Summary>(item: &T) {
    println!("Static Dispatch: {}", item.summarize());
}

// Task 2: Dynamic Dispatch (Trait Object)
fn print_summary_dynamic(item: &dyn Summary) {
    println!("Dynamic Dispatch: {}", item.summarize());
}

fn main() {
    let article = Article {
        title: String::from("Rust Mastery"),
        author: String::from("Expert"),
    };
    let text = String::from("Hello, Rust!");
    let number = 42;

    println!("--- Static Dispatch ---");
    print_summary_static(&article);
    print_summary_static(&text);
    print_summary_static(&number);

    println!("\n--- Dynamic Dispatch ---");
    // We use a Vec of Box<dyn Summary> to demonstrate polymorphism
    let list: Vec<Box<dyn Summary>> = vec![
        Box::new(article),
        Box::new(text),
        Box::new(number),
    ];

    for item in list {
        print_summary_dynamic(&*item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_summary() {
        let article = Article {
            title: String::from("Test Title"),
            author: String::from("Test Author"),
        };
        assert_eq!(article.summarize(), "Article: Test Title by Test Author");
    }

    #[test]
    fn test_string_summary() {
        let s = String::from("Hello");
        assert_eq!(s.summarize(), "String: Hello");
    }

    #[test]
    fn test_i32_summary() {
        let n = 100;
        assert_eq!(n.summarize(), "Number: 100");
    }
}
