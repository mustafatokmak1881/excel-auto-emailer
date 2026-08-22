use std::error::Error;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

pub async fn send(
    to: &str, 
    subject: &str, 
    html_body: &str, 
    attachments: Vec<(&str, &[u8])> // Dinamik ekler parametresi
) -> Result<(), Box<dyn Error>> {
    let my_email: &str = "mtkmk1881@gmail.com";
    
    // Mesaj oluşturucuyu başlatıyoruz
    let mut message = MessageBuilder::new()
        .from(("Excel Email Sender", my_email))
        .to(to)
        .subject(subject)
        .html_body(html_body);

    // Dinamik olarak gelen her dosyayı mesaja ekle
    for (file_name, file_data) in attachments {
        // Excel için genel MIME türü
        let mime_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
        
        // Mesajı güncelliyoruz
        message = message.attachment(mime_type, file_name, file_data);
    }

    SmtpClientBuilder::new("smtp.gmail.com", 587)?
        .implicit_tls(false)
        .credentials((my_email, "esgu bafl akfv bqzd"))
        .connect()
        .await?
        .send(message)
        .await?;

    Ok(())
}