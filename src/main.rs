//mod services;

//use services::mail::send;
use std::error::Error;
// use std::env;
use std::path::Path;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_dir : &str = "excelfilesx";

    let exists: bool = Path::new(my_dir).is_dir();
    if !exists {
        fs::create_dir(my_dir);
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
