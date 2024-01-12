use std::fs::File;
use std::io::Read;

fn main() {
    let greeting_file_result = File::open("unrecoverable_errors.rs");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut content = String::new();
    if let Err(error) = greeting_file.read_to_string(&mut content) {
        panic!("Problem reading the file: {:?}", error);
    }

    print!("{}", content);
}
