fn main() {
    // --- 1. Copy Trait (Stack Data) ---
    println!("--- 1. Copy Trait (Stack Data) ---");
    let x = 42;
    let y = x; // x의 값이 y로 '복사'됩니다.
    
    println!("x: {}, y: {}", x, y); 
    // x와 y는 각각 독립적인 스택 공간을 가지므로 둘 다 사용 가능합니다.

    // --- 2. Move Semantics (Heap Data) ---
    println!("\n--- 2. Move Semantics (Heap Data) ---");
    let s1 = String::from("hello");
    let s2 = s1; // s1의 소유권이 s2로 '이동(Move)'됩니다.

    // println!("s1: {}", s1); // [실습] 이 주석을 해제해보세요. 컴파일 에러가 발생합니다!
    println!("s2: {}", s2); // s2는 이제 새로운 소유자입니다.

    // --- 3. Clone Trait (Deep Copy) ---
    println!("\n--- 3. Clone Trait (Deep Copy) ---");
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3의 내용을 바탕으로 새로운 힙 메모리를 할당하여 '깊은 복사'를 수행합니다.

    println!("s3: {}, s4: {}", s3, s4); 
    // s3와 s4는 서로 다른 메모리 주소를 가지며, 둘 다 독립적으로 사용 가능합니다.

    // --- 4. Memory Address Check ---
    println!("\n--- 4. Memory Address Check ---");
    let s5 = String::from("address_test");
    let s6 = s5.clone();

    println!("s5의 데이터 주소: {:p}", s5.as_ptr());
    println!("s6의 데이터 주소: {:p}", s6.as_ptr());
    // 두 주소가 다르면 서로 다른 힙 메모리를 사용하고 있는 것입니다.

    // --- 5. Extra Credit: Vec<i32> (Copy) ---
    println!("\n--- 5. Extra Credit: Vec<i32> (Move) ---");
    let v1 = vec![1, 2, 3];
    let v2 = v1; // Vec은 힙 데이터를 가지므로 Move가 발생합니다.
    println!("v2: {:?}", v2);
    // println!("v1: {:?}", v1); // Error: use of moved value
}
