mod services;

use services::mail::send;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = send("mtkmk1881@gmail.com", "MErhabaaaa", "<marquee>Html Alanı buraya ya şş</marquee>").await;
    println!("Result: {:?}", result);

    Ok(())
}
