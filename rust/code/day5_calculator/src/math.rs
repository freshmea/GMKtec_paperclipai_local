
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err("Division by zero error".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}
