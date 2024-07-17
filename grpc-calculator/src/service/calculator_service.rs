use crate::proto;
pub use proto::calculator_server::{Calculator, CalculatorServer};

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();
        let response = proto::CalculationResponse {
            result: input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("Cannot Divide by 0!"));
        }

        let response = proto::CalculationResponse {
            result: input.a / input.b,
        };

        Ok(tonic::Response::new(response))
    }
}
