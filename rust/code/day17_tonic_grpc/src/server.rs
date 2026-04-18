use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{transport::Server, Request, Response, Status};

pub mod sensor {
    tonic::include_proto!("sensor");
}

use sensor::sensor_service_server::{SensorService, SensorServiceServer};
use sensor::{SensorReading, SensorRequest};

#[derive(Debug, Default)]
pub struct MySensorService {}

#[tonic::async_trait]
impl SensorService for MySensorService {
    type StreamSensorDataStream = Pin<Box<dyn Stream<Item = Result<SensorReading, Status>> + Send>>;

    async fn stream_sensor_data(
        &self,
        request: Request<SensorRequest>,
    ) -> Result<Response<Self::StreamSensorDataStream>, Status> {
        let sensor_id = request.into_inner().sensor_id;
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            for i in 1..=5 {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;

                let reading = SensorReading {
                    sensor_id: sensor_id.clone(),
                    value: i as f64 * 1.5,
                    timestamp,
                };

                if tx.send(Ok(reading)).await.is_err() {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream) as Self::StreamSensorDataStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let sensor_service = MySensorService::default();

    println!("SensorService listening on {}", addr);

    Server::builder()
        .add_service(SensorServiceServer::new(sensor_service))
        .serve(addr)
        .await?;

    Ok(())
}
