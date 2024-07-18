pub mod service;

use proto::calculator_server::CalculatorServer;
use service::calculator_service::{self, CalculatorService};
use std::time::Duration;
use tonic::transport::Server;
use tonic_health::server::HealthReporter;
use tonic_reflection;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

/// This function (somewhat improbably) flips the status of a service every second, in order
/// that the effect of `tonic_health::HealthReporter::watch` can be easily observed.
async fn twiddle_service_status(mut reporter: HealthReporter) {
    let mut iter = 0u64;
    loop {
        iter += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;

        if iter % 2 == 0 {
            reporter
                .set_serving::<CalculatorServer<CalculatorService>>()
                .await;
        } else {
            reporter
                .set_not_serving::<CalculatorServer<CalculatorService>>()
                .await;
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calc = calculator_service::CalculatorService::default();
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<CalculatorServer<CalculatorService>>()
        .await;

    tokio::spawn(twiddle_service_status(health_reporter.clone()));

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(health_service)
        .add_service(calculator_service::CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
