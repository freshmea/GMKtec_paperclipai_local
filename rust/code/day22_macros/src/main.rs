macro_rules! debug_print {
    // 패턴: 인자들을 쉼표로 구분하여 받음 ($($arg:expr),*)
    ($($arg:expr),*) => {
        {
            println!("--- Debug Print ---");
            $(
                // 각 인자에 대해 출력 (주의: expr 자체의 이름을 완벽히 알아내는 것은 
                // 기본 macro_rules만으로는 한계가 있으나, 여기서는 값 위주로 예시)
                println!("Value: {}", $arg);
            )*
            println!("-------------------");
        }
    };
}

// 좀 더 발전된 버전: 인자의 이름을 알고 싶다면 
// 보통은 별도의 crate나 복잡한 매크로가 필요하지만, 
// 여기서는 단순한 형태의 확장을 보여줍니다.
macro_rules! debug_print_simple {
    ($($arg:expr),*) => {
        {
            $(
                println!("arg = {:?}", $arg);
            )*
        }
    };
}

fn main() {
    let a = 10;
    let b = 20.5;
    let c = "Hello, Macros!";

    println!("Testing debug_print!");
    debug_print!(a, b, c);

    println!("\nTesting debug_print_simple!");
    debug_print_simple!(a, b, c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_expansion() {
        // 매크로는 출력을 수행하므로, 
        // 테스트에서는 값이 올바르게 계산되는지 확인하는 용도로 사용합니다.
        let x = 5;
        let y = 10;
        debug_print!(x, y);
        assert_eq!(x + y, 15);
    }
}
