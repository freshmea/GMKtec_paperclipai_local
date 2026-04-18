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
