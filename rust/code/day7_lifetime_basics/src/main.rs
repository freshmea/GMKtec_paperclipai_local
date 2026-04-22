// --- 1. Dangling Reference (에러 유도) ---
// 이 함수는 스코프가 끝나면 사라지는 지역 변수의 참조를 반환하려고 시도합니다.
/*
fn dangle() -> &String {
    let s = String::from("I am local");
    &s // 에러! s는 함수가 끝나면 드롭되므로, 이 참조는 허공을 가리키게 됩니다.
}
*/

// --- 2. Lifetime Annotation을 이용한 해결 ---
// 두 개의 문자열 슬라이스 중 더 긴 것을 반환하는 함수입니다.
// 'a는 입력받은 두 참조가 모두 최소한 'a만큼의 수명을 가져야 함을 의미합니다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// --- 3. Box<T> - 재귀적 구조 ---
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new(val: i32) -> Self {
        List::Cons(val, Box::new(List::Nil))
    }
}

// --- 4. Rc<T> - 공유 소유권 ---
use std::rc::Rc;

#[derive(Debug)]
struct SharedData {
    value: String,
}

fn main() {
    // --- 1. Lifetime 실습 ---
    println!("--- 1. Lifetime 실습 ---");
    let string1 = String::from("long string");
    let string2 = "short";

    let result = longest(string1.as_str(), string2);
    println!("가장 긴 문자열은: {}", result);

    let string3 = String::from("hello");
    let result2;
    {
        let string4 = String::from("world");
        result2 = longest(string3.as_str(), string4.as_str());
        println!("Scope 안의 결과: {}", result2);
    } 
    // result2는 string4의 수명에 묶여 있으므로, string4가 드롭된 후에는 사용할 수 없습니다.
    // 아래 println!은 컴파일 에러를 유도합니다.
    // println!("Scope 밖의 결과: {}", result2); 

    // --- 2. Box 실습 ---
    println!("\n--- 2. Box 실습 ---");
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("재귀적 리스트: {:?}", list);

    // --- 3. Rc 실습 ---
    println!("\n--- 3. Rc 실습 ---");
    let data = Rc::new(SharedData {
        value: String::from("Shared Knowledge"),
    });

    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);

    println!("Owner 1 sees: {}", owner1.value);
    println!("Owner 2 sees: {}", owner2.value);
    println!("Reference Count: {}", Rc::strong_count(&data));
}
