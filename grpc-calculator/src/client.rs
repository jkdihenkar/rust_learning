use std::error::Error;

use proto::calculator_client::CalculatorClient;

pub mod proto {
    tonic::include_proto!("calculator");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "http://[::1]:50051";

    let mut client = CalculatorClient::connect(url).await?;

    let creq = proto::CalculationRequest{
        a: 23,
        b: 32,
    };

    let request = tonic::Request::new(creq);

    let response = client.add(request).await?;

    println!("Got response: {:?}", response.get_ref().result);

    Ok(())
}
