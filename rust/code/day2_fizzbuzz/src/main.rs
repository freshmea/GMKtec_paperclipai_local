fn fizzbuzz(n: i32) -> &'static str {
    if n % 15 == 0 {
        "FizzBuzz"
    } else if n % 3 == 0 {
        "Fizz"
    } else if n % 5 == 0 {
        "Buzz"
    } else {
        "Not FizzBuzz"
    }
}

fn main() {
    println!("--- FizzBuzz Test ---");
    for i in 1..=20 {
        println!("{}: {}", i, fizzbuzz(i));
    }

    println!("\n--- Loop Test ---");
    let mut count = 0;
    loop {
        count += 1;
        if count > 5 {
            break;
        }
        println!("Loop count: {}", count);
    }

    println!("\n--- For Loop Test ---");
    let numbers = [10, 20, 30, 40, 50];
    for num in numbers.iter() {
        println!("Number: {}", num);
    }

    println!("\n--- If Expression Test ---");
    let number = 7;
    let result = if number < 10 {
        "Small number"
    } else {
        "Large number"
    };
    println!("The number {} is a {}", number, result);
}
