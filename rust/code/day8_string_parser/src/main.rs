use std::collections::HashMap;

/// Exercise 1: Simple CSV Parser
/// Takes a comma-separated string and returns a Vec of its parts.
fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

/// Exercise 2: Word Counter
/// Returns the count of words in a sentence.
fn count_words(sentence: &str) -> usize {
    sentence.split_whitespace().count()
}

/// Exercise 3: Key-Value Parsing
/// Parses a string in "key:value" format into a HashMap.
fn parse_key_value(input: &str) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();
    
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = trimmed.split_once(':') {
            map.insert(
                key.trim().to_string(),
                value.trim().to_string(),
            );
        } else {
            return Err(format!("Invalid line format: '{}'", trimmed));
        }
    }
    
    Ok(map)
}

fn main() {
    // --- Exercise 1: CSV ---
    println!("--- Exercise 1: CSV Parser ---");
    let csv_data = "name, age, city, occupation";
    let csv_parts = parse_csv_line(csv_data);
    println!("CSV parts: {:?}", csv_parts);

    // --- Exercise 2: Word Count ---
    println!("\n--- Exercise 2: Word Counter ---");
    let sentence = "   Rust is   a powerful systems language!   ";
    let count = count_words(sentence);
    println!("Sentence: '{}'", sentence);
    println!("Word count: {}", count);

    // --- Exercise 3: Key-Value ---
    println!("\n--- Exercise 3: Key-Value Parser ---");
    let config = r#"
        # This is a comment
        server: 127.0.0.1
        port: 8080
        protocol: https
        timeout: 30
    "#;

    match parse_key_value(config) {
        Ok(map) => {
            println!("Parsed Config:");
            for (k, v) in &map {
                println!("  {} => {}", k, v);
            }
            
            // Test accessing a specific value
            if let Some(port) = map.get("port") {
                println!("\nFound port: {}", port);
            }
        }
        Err(e) => println!("Error parsing config: {}", e),
    }

    // Test error case
    println!("\n--- Testing Error Case ---");
    let bad_config = "key_without_colon";
    match parse_key_value(bad_config) {
        Ok(_) => println!("Should have failed!"),
        Err(e) => println!("Correctly caught error: {}", e),
    }
}
