mod services;

use services::mail::send;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = send().await;
    println!("Result: {:?}", result);

    Ok(())
}
