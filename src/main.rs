//mod services;

//use services::mail::send;
use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let paths =  fs::read_dir(".")?;
    for path in paths {
        println!("dir: {}", path?.path().display());
    }

    //let result = send("mtkmk1881@gmail.com", "M3rhabaaaa", "<html style='background-color:grey'>Html Alanı buraya ya şş</html>").await;
    //println!("Result: {:?}", result);

    Ok(())
}
