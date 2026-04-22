# Day 6: Simple Shared Store

## Overview

This project demonstrates the core concepts of Rust's Ownership and Borrowing. It implements a simple data store that allows for reading (immutable borrowing) and updating (mutable borrowing) of a string.

## Learning Objectives

- Understand **Ownership** and how it prevents use-after-free errors.
- Practice **Immutable Borrowing** (`&T`) to read data without taking ownership.
- Practice **Mutable Borrowing** (`&mut T`) to modify data safely.
- Observe the **Borrow Checker** in action when attempting to violate borrowing rules (e.g., modifying while reading).

## How to Run

To run the demonstration, use:
```bash
cargo run
```

## Exercises

1. **Observe Move Semantics:** Try to use a variable after its value has been moved to another variable.
2. **Observe Immutable Borrowing:** Pass a reference to a function to read a value.
3. **Observe Mutable Borrowing:** Pass a mutable reference to a function to modify a value.
4. **Trigger a Borrow Checker Error:** Uncomment the code in `src/main.rs` that attempts to mutate a value while an immutable reference is still active.
