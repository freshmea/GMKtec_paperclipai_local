use sensor::sensor_service_client::SensorServiceClient;
use sensor::SensorRequest;
use tokio_stream::StreamExt;

pub mod sensor {
    tonic::include_proto!("sensor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SensorServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SensorRequest {
        sensor_id: "sensor-001".into(),
    });

    println!("Requesting stream from server...");
    let mut stream = client.stream_sensor_data(request).await?.into_inner();

    while let Some(reading) = stream.next().await {
        match reading {
            Ok(r) => println!(
                "Received: [Sensor: {}] [Value: {:.2}] [Timestamp: {}]",
                r.sensor_id, r.value, r.timestamp
            ),
            Err(e) => eprintln!("Error receiving stream: {}", e),
        }
    }

    println!("Stream finished.");
    Ok(())
}
