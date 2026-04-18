use metrics::{counter, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use rand::Rng;
use std::time::Instant;
use tracing::{error, info, instrument, Span};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // 1. Tracing 설정
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    // 2. Metrics 설정 (Prometheus exporter)
    let builder = PrometheusBuilder::new();
    let handle = builder
        .install_recorder()
        .expect("failed to install recorder");

    info!("Observability 서버가 시작되었습니다.");

    // 메인 루프 시뮬레이션
    for i in 1..=5 {
        info!(iteration = i, "새로운 요청 시뮬레이션 시작");
        process_request(i).await;
        
        // Prometheus 메트릭 출력을 위해 잠시 대기
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        
        // 현재 메트릭 상태 출력 (실제 운영 환경에서는 Prometheus가 scrape 함)
        let metric_report = handle.render();
        println!("\n--- 현재 Prometheus 메트릭 상태 ---\n{}\n", metric_report);
    }

    info!("시뮬레이션 종료");
}

#[instrument]
async fn process_request(request_id: i32) {
    let start = Instant::now();
    
    // 요청 카운터 증가
    counter!("http_requests_total", "request_id" => request_id.to_string()).increment(1);

    info!(request_id = request_id, "요청 처리 중...");

    // 작업 시간 시뮬레이션 (랜덤한 지연)
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(100..500);
    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;

    if request_id % 3 == 0 {
        error!(request_id = request_id, "요청 처리 중 오류 발생!");
    } else {
        info!(request_id = request_id, "요청 성공적으로 처리됨");
    }

    // 레이턴시 히스토그램 기록
    let duration = start.elapsed().as_secs_f64();
    histogram!("http_request_duration_seconds").record(duration);
}
