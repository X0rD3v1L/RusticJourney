use std::borrow::Cow;

fn main() {
    let cow:Cow<str> = Cow::Borrowed("Hello");
    let mut owned = cow.into_owned();
    owned.push_str(", World!");
    println!("{}", owned);
}