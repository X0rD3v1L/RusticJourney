fn main() {
    let name = "ben";

    let mut rev = String::new();

    for i in (0..name.len()).rev() {
        rev.push(name.chars().nth(i).unwrap());
    }

    println!("Original : {}", name);
    println!("Reversed : {}", rev);
}