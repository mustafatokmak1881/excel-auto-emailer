use std::error::Error;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

pub async fn send(to: &str, subject: &str, html_body: &str) -> Result<(), Box<dyn Error>> {
    let my_email: &str = "mtkmk1881@gmail.com";
    let message = MessageBuilder::new()
        .from(("Excel Email Sender", my_email))
        .to(to)
        .subject(subject)
        .html_body(html_body);

    SmtpClientBuilder::new("smtp.gmail.com", 587)?
        .implicit_tls(false)
        .credentials((my_email, "esgu bafl akfv bqzd"))
        .connect()
        .await?
        .send(message)
        .await?;

    Ok(())
}