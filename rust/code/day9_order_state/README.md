# Day 9: Order State (State Pattern with Enums)

## Overview

In this module, you will explore how to model complex state transitions using Rust's powerful `enum` and pattern matching. You will implement an Order Management System where an order's state dictates what actions can be performed on it.

## Key Concepts

1.  **State Machine Pattern**: Using enums to represent different states of an object (e.g., `Pending`, `Paid`, `Shipped`, `Delivered`).
2.  **Pattern Matching**: Using `match` to enforce valid transitions between states.
3.  **Data-Carrying Enums**: Storing state-specific data within enum variants (e.g., storing a tracking number only when the state is `Shipped`).
4.  **Encapsulation**: Ensuring that state transitions can only happen through controlled methods.

## Exercises

### Exercise 1: Order State Transitions

Create an `Order` struct and an `OrderState` enum. Implement methods to transition from `Pending` $\rightarrow$ `Paid` $\rightarrow$ `Shipped` $\rightarrow$ `Delivered`. Ensure that invalid transitions (e.g., `Pending` $\rightarrow$ `Shipped`) are handled with errors.

### Exercise 2: Carrying Data in States

Extend the `Shipped` state to carry a `tracking_number: String`. Implement a method to retrieve this tracking number only if the order is in the `Shipped` state.

## Lab Instructions

1.  Navigate to the directory: `cd day9_order_state`
2.  Open `src/main.rs` and implement the state machine.
3.  Run the code to verify: `cargo run`

## Implementation Example (Snippet)

```rust
enum OrderState {
    Pending,
    Paid,
    Shipped { tracking_number: String },
    Delivered,
}

struct Order {
    id: u32,
    state: OrderState,
}

impl Order {
    fn ship(&mut self, tracking: String) -> Result<(), String> {
        match self.state {
            OrderState::Paid => {
                self.state = OrderState::Shipped { tracking_number: tracking };
                Ok(())
            }
            _ => Err("Order must be Paid before it can be Shipped.".to_string()),
        }
    }
}

fn main() {
    let mut order = Order { id: 1, state: OrderState::Pending };
    
    match order.ship("ABC-123".to_string()) {
        Ok(_) => println!("Shipped!"),
        Err(e) => println!("Error: {}", e),
    }
}
```
