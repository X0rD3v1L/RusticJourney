struct Book<'a> {
    title: &'a str,
}
impl<'a> Book<'a> {
    fn new(title: &'a str) -> Book<'a> {
        Book { title }
    }
    fn title(&self) -> &str {
        self.title
    }
}
fn main() {
    let my_book = Book::new("Rust Programming");
    println!("{}", my_book.title());
}