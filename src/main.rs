use std::error::Error;
use std::path::Path;
use std::fs;

// Eğer send fonksiyonunuz başka bir modüldeyse onu import edin:
mod services;
use services::mail::send;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_dir: &str = "excel-dosyalari-buraya";
    let my_email_list: &str = "eposta-listesi.txt";

    let excel_dir: bool = Path::new(my_dir).is_dir();
    let email_list_file: bool = fs::exists(my_email_list)?;

    if !excel_dir {
        let _ = fs::create_dir(my_dir);
    }

    if !email_list_file {
        let _ = fs::File::create(my_email_list);
    }

    let email_list: String = fs::read_to_string(my_email_list)?;
    let binding = &email_list;
    let emails: Vec<&str> = binding.split("\r\n").filter(|s| !s.is_empty()).collect();
    println!("Email list: {:?}", emails);

    let paths = fs::read_dir(my_dir)?;

    // Eklenecek dosyaları tutacağımız dinamik liste: (Dosya Adı, Dosya İçeriği)
    let mut attachments: Vec<(String, Vec<u8>)> = Vec::new();

    for path in paths {
        let entry = path?;
        let excel_file_path = entry.path();
        
        // Sadece dosya olanları ve .xls / .xlsx uzantılıları filtreleyelim
        if excel_file_path.is_file() {
            if let Some(file_name_str) = excel_file_path.file_name().and_then(|n| n.to_str()) {
                if file_name_str.contains(".xls") {
                    // Dosyanın ham baytlarını oku
                    let excel_file_bytes = fs::read(&excel_file_path)?;
                    attachments.push((file_name_str.to_string(), excel_file_bytes));
                }
            }
        }
    }

    // Bulunan Excel dosyalarını konsola yazdıralım
    println!("Toplam bulunan Excel dosyası: {}", attachments.len());

    // Örnek: Her bir maile bu dosyaları ekleyerek gönderim yapma döngüsü
    for email in emails {
        if email.trim().is_empty() { continue; }
        
        // Referansları `send` fonksiyonuna gönderiyoruz
        let attach_refs: Vec<(&str, &[u8])> = attachments
            .iter()
            .map(|(name, data)| (name.as_str(), data.as_ref()))
            .collect();

        let result = send(
            email, 
            "Excel Raporunuz", 
            "<html style='background-color:grey'>Merhabalar, ekte Excel dosyalarınız bulunmaktadır.</html>", 
            attach_refs
        ).await;

        println!("Mail gönderildi ({}): {:?}", email, result);
    }

    Ok(())
}