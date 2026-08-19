use std::error::Error;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

pub async fn send() -> Result<(), Box<dyn Error>> {
    let my_email: &str = "mtkmk1881@gmail.com";
    let message = MessageBuilder::new()
        .from(("Mustafa Tkm", my_email))
        .to(my_email)
        .subject("Derken patır kütür ya şş")
        .text_body("Nasılsın ?\r\n")
        .html_body("<p>iyi misin ?</p>");

    SmtpClientBuilder::new("smtp.gmail.com", 587)?
        .implicit_tls(false)
        .credentials((my_email, "esgu bafl akfv bqzd"))
        .connect()
        .await?
        .send(message)
        .await?;

    Ok(())
}