// --- 1. Declarative Macros (macro_rules!) ---
// 매크로는 코드를 생성하는 코드를 작성하는 것입니다. 
// 중복되는 패턴을 줄이고, DSL(Domain Specific Language)을 만들 수 있습니다.

macro_rules! vec_and_print {
    // 패턴 매칭: 변수들을 인자로 받음
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("Added: {}", $x);
            )*
            temp_vec
        }
    };
}

macro_rules! calculate {
    // 단순 연산 매크로
    (add $a:expr, $b:expr) => { $a + $b };
    (sub $a:expr, $b:expr) => { $a - $b };
    (mul $a:expr, $b:expr) => { $a * $b };
    (div $a:expr, $b:expr) => { $a / $b };
}

// --- 2. Procedural Macros (Concept) ---
// declarative macro와 달리, procedural macro는 함수처럼 컴파일 타임에 코드를 조작합니다.
// derive, attribute, function-like 매크로 세 종류가 있습니다.
// (이 예제에서는 코드 설명으로 대체합니다.)

fn main() {
    println!("--- 1. Declarative Macros (macro_rules!) ---");
    
    // vec_and_print! 매크로 사용
    let my_vec = vec_and_print![10, 20, 30, 40];
    println!("Final vector: {:?}", my_vec);

    println!("\n--- 2. Simple Arithmetic Macro ---");
    let sum = calculate!(add 10, 5);
    let diff = calculate!(sub 10, 5);
    let prod = calculate!(mul 10, 5);
    let quot = calculate!(div 10, 5);

    println!("10 + 5 = {}", sum);
    println!("10 - 5 = {}", diff);
    println!("10 * 5 = {}", prod);
    println!("10 / 5 = {}", quot);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_and_print() {
        let v = vec_and_print![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate!(add 10, 5), 15);
        assert_eq!(calculate!(sub 10, 5), 5);
        assert_eq!(calculate!(mul 10, 5), 50);
        assert_eq!(calculate!(div 10, 5), 2);
    }
}
