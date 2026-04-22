# Day 7: Lifetime Basics

## Overview

This project explores Rust's advanced memory management concepts: Lifetimes, `Box<T>`, and `Rc<T>`. It demonstrates how to ensure reference validity, manage heap allocation for recursive structures, and implement shared ownership.

## Learning Objectives

- **Lifetimes:** Understand how to use lifetime annotations (`'a`) to guarantee that references remain valid.
- **Box<T>:** Learn how to allocate data on the heap and define recursive data structures.
- **Rc<T>:** Implement shared ownership using reference counting.

## How to Run

To run the lifetime demonstration, use:
```bash
cargo run
```

## Exercises

1. **Lifetime Annotation:** Observe how the `longest` function requires lifetime annotations to satisfy the borrow checker.
2. **Recursive Structure with Box:** Examine how `Box` allows the `Node` enum to contain itself.
3. **Shared Ownership with Rc:** See how multiple owners can hold a reference to the same data using `Rc`.
