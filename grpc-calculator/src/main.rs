pub mod service;

use service::calculator_service;
use tonic::transport::Server;
use tonic_health::server::HealthReporter;
use tonic_reflection;

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calc = calculator_service::CalculatorService::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(service)
        .add_service(calculator_service::CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
