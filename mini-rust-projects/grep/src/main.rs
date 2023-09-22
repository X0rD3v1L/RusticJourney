#![allow(unused)]
use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}


fn main(){
    let args = Cli::parse();

    /// Outputting the arguments passed
    println!("{}",args.pattern);
    println!("{:?}",args.path);

    let file = match File::open(&args.path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}",e);
            return;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(&args.pattern){
                println!("{}", line);
            }
        } else {
            eprintln!("Error reading line from the file");
        }
    }

}