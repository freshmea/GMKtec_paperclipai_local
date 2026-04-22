# Day 28: Generic Associated Types (GATs)

## Objective
Understand how Generic Associated Types (GATs) allow associated types to have their own generic parameters, such as lifetimes, enabling more powerful abstractions like Async Iterators.

## Key Concepts
- **Generic Associated Types (GATs):** Associated types that can take parameters (e.g., `type Iter<'a>`).
- **Async Iterators:** Using GATs to define an iterator that returns a `Future` which is tied to the lifetime of the iterator itself.
- **Lifetime Elision & Bounds:** How GATs interact with lifetimes and trait bounds.

## Lab Instructions
1. Open `src/main.rs`.
2. Examine the `AsyncIterator` trait definition and how the `Iter<'a>` associated type is defined with a lifetime parameter.
3. Study the `CounterFuture` implementation and how it uses the lifetime `'a` to reference the `Counter`.
4. Run the code to see the demo output: `cargo run`.
5. **Challenge:** Try to implement a truly functional `AsyncIterator` using `std::cell::Cell` to allow mutation through an immutable reference in `iter(&self)`.

## Running the Code
```bash
cargo run
```

## Testing
```bash
cargo test
```
