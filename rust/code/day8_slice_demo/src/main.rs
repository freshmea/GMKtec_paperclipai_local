fn main() {
    // --- 1. Array Slicing ---
    println!("--- 1. Array Slicing ---");
    let arr = [10, 20, 30, 40, 50, 60, 70];
    println!("Original array: {:?}", arr);

    // Slice from index 1 to 4 (exclusive of 4)
    let slice1 = &arr[1..4];
    println!("Slice [1..4]: {:?}", slice1);

    // Slice from index 3 to end
    let slice2 = &arr[3..];
    println!("Slice [3..]: {:?}", slice2);

    // Slice using range syntax
    let slice3 = &arr[..3];
    println!("Slice [..3]: {:?}", slice3);

    // --- 2. String Slicing ---
    println!("\n--- 2. String Slicing ---");
    let s = String::from("Hello, Rust World!");
    println!("Original string: {}", s);

    // Slicing a string
    let hello = &s[0..5];
    println!("First 5 chars: {}", hello);

    let rust = &s[7..11];
    println!("'Rust' part: {}", rust);

    // --- 3. UTF-8 Safety (Demonstration) ---
    println!("\n--- 3. UTF-8 Safety ---");
    let utf8_s = String::from("🦀 Rust");
    println!("UTF-8 string: {}", utf8_s);
    
    // Note: Slicing by byte index must land on character boundaries.
    // The crab emoji '🦀' takes 4 bytes.
    let emoji_slice = &utf8_s[0..4];
    println!("Emoji slice: {}", emoji_slice);

    // The following would PANIC because index 1 is in the middle of the 🦀 emoji
    // let bad_slice = &utf8_s[1..4]; 
    // println!("{}", bad_slice);

    // --- 4. Parsing with Slices and Iterators ---
    println!("\n--- 4. Parsing with Slices & Iterators ---");
    let numbers_str = "10 20 30 40 50";
    println!("Input string: '{}'", numbers_str);

    // Split by whitespace, parse each part into i32, and collect into a Vec
    let parsed_numbers: Vec<i32> = numbers_str
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse number"))
        .collect();

    println!("Parsed Vec<i32>: {:?}", parsed_numbers);
    println!("Sum of numbers: {}", parsed_numbers.iter().sum::<i32>());
}
