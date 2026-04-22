
pub struct DataProcessor;

impl DataProcessor {
    pub fn filter_pattern<'a>(input: &'a Vec<String>, pattern: &str) -> Vec<&'a str> {
        input
            .iter()
            .map(|s| s.as_str())
            .filter(|s| s.contains(pattern))
            .collect()
    }
}
