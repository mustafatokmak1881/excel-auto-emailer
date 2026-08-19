use std::error::Error;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

pub async fn send() -> Result<(), Box<dyn Error>> {
    let message = MessageBuilder::new()
        .from(("Mustafa Tkm", "mtkmk1881@gmail.com"))
        .to("mtkmk1881@gmail.com")
        .subject("Naber Cınım")
        .text_body("Nasılsın ?\r\n")
        .html_body("<p>iyi misin ?</p>");

    SmtpClientBuilder::new("smtp.gmail.com", 587)?
        .implicit_tls(false)
        .credentials(("mtkmk1881@gmail.com", "esgu bafl akfv bqzd"))
        .connect()
        .await?
        .send(message)
        .await?;

    Ok(())
}