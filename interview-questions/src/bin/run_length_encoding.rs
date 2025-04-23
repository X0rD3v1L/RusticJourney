fn main() {
    let name = String::from("aabbbaacc");
    let chars:Vec<char> = name.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let mut count = 1;

        while i + 1 < chars.len() && chars[i] == chars[i + 1] {
            i += 1;
            count += 1;
        }

        print!("{}{}", count, chars[i]);
        i += 1;
    }
    println!();
}
//2a3b2a2c
