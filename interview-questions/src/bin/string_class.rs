use std::fmt;

struct MyString {
    data: Vec<char>,
}

impl MyString {
    fn from(s: &str) -> Self {
        Self { data: s.chars().collect() }
    }

    fn push(&mut self, ch: char) {
        self.data.push(ch);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_str(&self) -> String {
        self.data.iter().collect()
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in &self.data {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

fn main() {
    let mut my_str = MyString::from("Hello");
    my_str.push(',');
    my_str.push(' ');
    my_str.push('🌍');

    println!("MyString: {}", my_str);
    println!("Length: {}", my_str.len());
    println!("As real string: {}", my_str.as_str());
}
