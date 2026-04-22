# Day 8: String Parser (String Manipulation & Slices)

## Overview

In this module, you will dive deeper into string manipulation in Rust. You'll learn how to use slices to parse strings, split them into parts, and handle text data efficiently without unnecessary allocations.

## Key Concepts

1.  **String Parsing**: The process of analyzing a string to extract meaningful data.
2.  **`split()`, `split_whitespace()`, `split_once()`**: Methods used to break a string into smaller components based on delimiters.
3.  **Iterators**: Using iterators to process the parts of a string lazily and efficiently.
4.  **Error Handling in Parsing**: Using `Option` or `Result` to handle cases where the string might not match the expected format.

## Exercises

### Exercise 1: Simple CSV Parser

Write a function that takes a string representing a single line of a CSV (e.g., `"name,age,city"`) and returns a `Vec<String>` containing the split parts.

### Exercise 2: Word Counter

Write a function that takes a sentence and returns the number of words in it using `split_whitespace()`.

### Exercise 3: Key-Value Parsing (Advanced)

Parse a string in the format `key:value` into a `HashMap<String, String>`.

## Lab Instructions

1.  Navigate to the directory: `cd day8_string_parser`
2.  Open `src/main.rs` and implement the exercises.
3.  Run the code to verify: `cargo run`

## Implementation Example (Snippet)

```rust
fn main() {
    let data = "apple,banana,cherry";
    let fruits: Vec<&str> = data.split(',').collect();
    println!("Fruits: {:?}", fruits);

    let sentence = "Rust is awesome";
    let word_count = sentence.split_whitespace().count();
    println!("Word count: {}", word_count);
}
```
