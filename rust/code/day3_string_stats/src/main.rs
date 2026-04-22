fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_chars(text: &str) -> usize {
    text.chars().count()
}

fn count_occurrences(text: &str, target: &str) -> usize {
    text.matches(target).count()
}

fn main() {
    let sample_text = String::from("Rust is a systems programming language that focuses on safety, speed, and concurrency.");
    let slice: &str = &sample_text[0..15]; // "Rust is a syst"

    println!("--- String Stats ---");
    println!("Original text: {}", sample_text);
    println!("Slice [0..15]: {}", slice);
    println!("Word count: {}", count_words(&sample_text));
    println!("Char count: {}", count_chars(&sample_text));
    println!("Occurrence of 'Rust': {}", count_occurrences(&sample_text, "Rust"));

    println!("\n--- UTF-8 & Multibyte Test ---");
    let korean_text = "안녕하세요 Rust";
    println!("Korean text: {}", korean_text);
    println!("Char count (chars()): {}", korean_text.chars().count());
    println!("Byte count (len()): {}", korean_text.len());
    
    // Demonstrating safe slicing with chars()
    let first_word: String = korean_text.chars().take(5).collect();
    println!("First 5 characters: {}", first_word);
}
