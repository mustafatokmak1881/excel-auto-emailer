//mod services;

//use services::mail::send;
use std::error::Error;
// use std::env;
use std::path::Path;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_dir: &str = "excel-dosyalari-buraya";
    let my_email_list: &str =  "eposta-listesi.txt";

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
    let emails: Vec<&str> = binding.split("\r\n").collect();
    println!("Email list: {:?}", emails);

    let paths = fs::read_dir(my_dir)?;

    for path in paths {
        let excel_file_path: &str = &path?.path().display().to_string();
        let is_excel_file: bool = excel_file_path.contains(".xls");

        if is_excel_file {
            let excel_file = fs::read(excel_file_path)?;
            //println!("excel file: {:?}", excel_file);
        }
     
    }

    //let result = send("mtkmk1881@gmail.com", "M3rhabaaaa", "<html style='background-color:grey'>Html Alanı buraya ya şş</html>").await;
    //println!("Result: {:?}", result);

    Ok(())
}
