fn main() {
    // 1. 불변 변수 (기본값)
    let x = 5;
    println!("x is: {}", x);

    // 2. 가변 변수
    let mut y = 10;
    println!("y is: {}", y);
    y = 15;
    println!("y is now: {}", y);

    // 3. 섀도잉 (Shadowing)
    let spaces = "   "; // 문자열 타입
    let spaces = spaces.len(); // 숫자 타입으로 섀도잉
    println!("Number of spaces: {}", spaces);

    // 4. 튜플과 배열
    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // 패턴 매칭 분해
    println!("a: {}, b: {}, c: {}", a, b, c);

    // 배열
    let arr = [1, 2, 3, 4, 5];
    println!("First element: {}", arr[0]);
}
