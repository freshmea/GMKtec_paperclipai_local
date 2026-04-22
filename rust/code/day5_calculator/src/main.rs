mod math;

use math::{calculate, Operation};
use std::io::{self, Write};

fn main() {
    println!("--- Enhanced Calculator (Day 5) ---");

    loop {
        print!("\nEnter first number (or 'q' to quit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "q" { break; }
        let a: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid number"); continue; }
        };

        print!("Enter operator (+, -, *, /): ");
        io::stdout().flush().unwrap();
        let mut op_input = String::new();
        io::stdin().read_line(&mut op_input).unwrap();
        let op = match op_input.trim() {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => { println!("Invalid operator"); continue; }
        };

        print!("Enter second number: ");
        io::stdout().flush().unwrap();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        let b: f64 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid number"); continue; }
        };

        match calculate(a, b, op) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("Goodbye!");
}
