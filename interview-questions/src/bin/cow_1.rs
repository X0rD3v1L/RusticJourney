use std::borrow::Cow;

/*
pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
 {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
*/
fn remove_spaces<'a>(input: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
    let input = input.into();
    if input.contains(' ') {
        Cow::Owned(input.replace(" ", ""))
    } else {
        input
    }
}


fn main() {
    let s1 = "HelloWorld"; // &str
    let s2 = String::from("Hello World"); // String

    let r1 = remove_spaces(s1);
    let r2 = remove_spaces(s2); // works!

    println!("r1: {}", r1);
    println!("r2: {}", r2);
}


