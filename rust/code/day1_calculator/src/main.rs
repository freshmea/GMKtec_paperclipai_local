fn main() {
    let num1 = 10.0;
    let num2 = 5.0;
    let operator = '+';

    let result = calculate(num1, num2, operator);
    println!("{} {} {} = {}", num1, operator, num2, result);
}

fn calculate(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero!");
                0.0
            }
        },
        _ => {
            println!("Error: Invalid operator!");
            0.0
        }
    }
}
