// --- 1. Parallel Iteration with Rayon ---
// Rayon은 데이터 병렬 처리를 위한 라이점입니다. 
// 반복문(iterator)을 아주 쉽게 병렬로 변환할 수 있게 해줍니다.
use rayon::prelude::*;

fn parallel_sum(numbers: Vec<i32>) -> i32 {
    // par_iter()를 사용하면 내부적으로 작업을 여러 스레드에 분산합니다.
    numbers.par_iter().sum()
}

fn parallel_map_square(numbers: Vec<i32>) -> Vec<i32> {
    // par_iter()와 map()을 조합하여 병렬로 연산을 수행합니다.
    numbers.par_iter().map(|&x| x * x).collect()
}

// --- 2. Work Stealing Concept ---
// Rayon은 'Work Stealing' 알고리즘을 사용합니다.
// 한 스레드가 할 일이 끝나면, 아직 할 일이 많은 다른 스레드의 큐에서 작업을 훔쳐와서 처리함으로써
// CPU 자원을 효율적으로 활용합니다.

fn main() {
    println!("--- 1. Parallel Iteration with Rayon ---");
    
    let numbers: Vec<i32> = (1..1_000_000).collect();

    // 병렬 합계 계산
    let sum = parallel_sum(numbers.clone());
    println!("Parallel Sum: {}", sum);

    // 병렬 제곱 계산
    let squares = parallel_map_square(numbers);
    println!("First 5 squares: {:?}", &squares[..5]);
    println!("Last 5 squares: {:?}", &squares[squares.len()-5..]);

    println!("\n--- 2. Performance Note ---");
    println!("Rayon은 데이터가 충분히 크고 연산 비용이 높을 때 가장 효과적입니다.");
    println!("작은 데이터셋에서는 스레드 생성 및 관리 비용 때문에 오히려 느려질 수 있습니다.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(parallel_sum(nums), 15);
    }

    #[test]
    fn test_parallel_map_square() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(parallel_map_square(nums), vec![1, 4, 9, 16, 25]);
    }
}
