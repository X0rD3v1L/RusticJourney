use std::fs::File;
use std::io::{self, Read};
fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() {
    match read_file() {
        Ok(text) => println!("{}", text),
        Err(err) => println!("Error: {}", err),
    }
}