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

    let paths = fs::read_dir(my_dir)?;
    for path in paths {
        let item: String = path?.path().display().to_string();
        println!("dir: {}", item);
    }

    //let result = send("mtkmk1881@gmail.com", "M3rhabaaaa", "<html style='background-color:grey'>Html Alanı buraya ya şş</html>").await;
    //println!("Result: {:?}", result);

    Ok(())
}
