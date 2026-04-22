# Day 8: Slice Demo (Slices & String Slices)

## Overview

This module focuses on **Slices**, which are a way to reference a contiguous sequence of elements in a collection rather than the entire collection. Understanding slices is crucial for efficient memory usage and for working with strings and arrays in Rust.

## Key Concepts

1.  **Slices (`&[T]`)**: A view into a collection that provides a reference to a contiguous sequence of elements. It consists of a pointer to the start and a length.
2.  **String Slices (`&str`)**: A specific type of slice used for strings. It is a reference to a portion of a `String` or a string literal.
3.  **Safety**: Slices are guaranteed to be valid as long as the original collection is not modified in a way that invalidates the slice (e.g., reallocating a `Vec` or `String`).

## Exercises

### Exercise 1: Array Slices

Create an array of integers and create slices of different lengths and offsets. Print the elements within those slices.

### Exercise 2: String Slicing

Take a `String` and extract specific parts of it (e.g., the first word, a substring) using slicing. Observe how UTF-8 character boundaries affect slicing.

## Lab Instructions

1.  Navigate to the directory: `cd day8_slice_demo`
2.  Open `src/main.rs` and implement the exercises.
3.  Run the code to verify: `cargo run`

## Implementation Example (Snippet)

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4]; // Elements at index 1, 2, 3
    println!("Slice: {:?}", slice);

    let s = String::from("Hello, world!");
    let hello = &s[0..5];
    println!("Slice of string: {}", hello);
}
```
