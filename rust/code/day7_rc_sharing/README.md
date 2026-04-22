# Day 7: Rc Sharing (Reference Counting)

## Overview

This module covers **`Rc<T>` (Reference Counting)**, a smart pointer that allows multiple ownership of data on the heap. This is essential when you need to share data across different parts of your program without worrying about a single owner's lifecycle.

## Key Concepts

1.  **`Rc<T>` (Reference Counted)**: A smart pointer that keeps track of the number of references to a value. When the last `Rc` pointer is dropped, the data is cleaned up from the heap.
2.  **Reference Count**: An integer maintained by `Rc` that increments when you clone the pointer and decrements when a pointer is dropped.
3.  **Immutability**: By default, `Rc<T>` only allows immutable access to the data it wraps. To mutate data inside an `Rc`, you would need additional wrappers like `RefCell<T>`.

## Exercises

### Exercise 1: Shared Ownership

Create a `String` wrapped in an `Rc`. Use `Rc::clone` to create multiple owners for that same string. Print the `strong_count` at each step to see how the reference count changes.

## Lab Instructions

1.  Navigate to the directory: `cd day7_rc_sharing`
2.  Open `src/main.rs` and implement the exercise.
3.  Run the code to verify: `cargo run`
4.  Use `Rc::strong_count(&your_rc_variable)` to inspect the current count.

## Implementation Example (Snippet)

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("Shared Data"));
    println!("Count: {}", Rc::strong_count(&data));

    let owner2 = Rc::clone(&data);
    println!("Count after clone: {}", Rc::strong_count(&data));
}
```
