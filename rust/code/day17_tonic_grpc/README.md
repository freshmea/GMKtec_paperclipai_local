# Day 17: gRPC & Tonic 실습 가이드

이 프로젝트는 Tonic 프레임워크를 사용하여 gRPC 서버와 클라이언트를 구현하는 예제입니다. 서버는 센서 데이터를 스트리밍하고, 클라이언트는 이를 수신합니다.

## 실행 방법 (How to Run)

이 예제는 서버와 클라이언트가 모두 필요합니다. 두 개의 터미널을 열어 실행하세요.

1.  **서버 실행 (Terminal 1):**
    ```bash
    cd /home/aa/vllm/rust/code/day17_tonic_grpc
    cargo run --bin server
    ```

2.  **클라이언트 실행 (Terminal 2):**
    ```bash
    cd /home/aa/vllm/rust/code/day17_tonic_grpc
    cargo run --bin client
    ```

## API 사용법 (API Usage)

### gRPC 서비스: `SensorService`

* **Method:** `StreamSensorData` (Server Streaming)
* **Request:** `SensorRequest { sensor_id: String }`
* **Response:** `stream SensorReading`

## 학습 포인트 (Learning Points)

* **Tonic & Protobuf:** `.proto` 파일이 어떻게 Rust 코드로 변환되어 `src/sensor.rs` (또는 포함된 코드)로 사용되는지 이해합니다.
* **Streaming RPC:** 서버가 클라이언트에게 지속적으로 데이터를 밀어넣는(Push) 스트리밍 방식의 동작 원리를 확인합니다.
* **Async Streams:** `tokio-stream`을 사용하여 비동기 스트림을 어떻게 소비(consume)하는지 학습합니다.
* **Concurrency:** 서버에서 `tokio::spawn`을 사용하여 스트리밍 로직을 별도 태스크로 실행하는 패턴을 확인합니다.
