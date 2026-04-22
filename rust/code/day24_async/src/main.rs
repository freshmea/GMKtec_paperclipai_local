// --- 1. Basic Async/Await with Tokio ---
// 비동기 프로그래밍은 단일 스레드 또는 소수의 스레드에서 
// I/O 대기 시간 동안 다른 작업을 수행할 수 있게 하여 효율성을 극대화합니다.

async fn fetch_data(id: i32) -> String {
    // 실제 환경에서는 네트워크 요청이 들어가는 부분입니다.
    // tokio::time::sleep을 사용하여 비동기 대기를 시뮬레이션합니다.
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    format!("Data from source {}", id)
}

async fn run_sequential() {
    println!("Starting sequential tasks...");
    let start = std::time::Instant::now();

    let res1 = fetch_data(1).await;
    let res2 = fetch_data(2).await;

    println!("Result 1: {}, Result 2: {}", res1, res2);
    println!("Sequential took: {:?}", start.elapsed());
}

async fn run_concurrent() {
    println!("\nStarting concurrent tasks...");
    let start = std::time::Instant::now();

    // tokio::join!은 여러 Future를 동시에 실행하고 모두 완료될 때까지 기다립니다.
    let (res1, res2) = tokio::join!(fetch_data(1), fetch_data(2));

    println!("Result 1: {}, Result 2: {}", res1, res2);
    println!("Concurrent took: {:?}", start.elapsed());
}

// --- 2. Spawning Tasks (Green Threads) ---
// tokio::spawn은 새로운 비동기 작업을 런타임의 스케줄러에 던집니다.
// 이는 작업 간의 독립성을 보장하며 병렬 실행을 가능하게 합니다.

async fn spawn_tasks() {
    println!("\nStarting spawned tasks...");
    let start = std::time::Instant::now();

    let handle1 = tokio::spawn(async {
        fetch_data(3).await
    });

    let handle2 = tokio::spawn(async {
        fetch_data(4).await
    });

    // JoinHandle을 통해 작업 완료를 기다리고 결과를 가져옵니다.
    let res1 = handle1.await.unwrap();
    let res2 = handle2.await.unwrap();

    println!("Result 3: {}, Result 4: {}", res1, res2);
    println!("Spawned tasks took: {:?}", start.elapsed());
}

#[tokio::main]
async fn main() {
    println!("--- Async/Await with Tokio ---");
    
    run_sequential().await;
    run_concurrent().await;
    spawn_tasks().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_data() {
        let res = fetch_data(10).await;
        assert_eq!(res, "Data from source 10");
    }

    #[tokio::test]
    async fn test_concurrent_execution() {
        let start = std::time::Instant::now();
        let (res1, res2) = tokio::join!(fetch_data(1), fetch_data(2));
        let elapsed = start.elapsed();

        assert_eq!(res1, "Data from source 1");
        assert_eq!(res2, "Data from source 2");
        // 두 작업이 병렬로 실행되었으므로 500ms보다 훨씬 짧아야 합니다 (약 500ms + overhead)
        // 하지만 실제로는 500ms 근처여야 함 (두 작업이 동시에 시작되었으므로)
        assert!(elapsed < std::time::Duration::from_millis(1000));
    }
}
