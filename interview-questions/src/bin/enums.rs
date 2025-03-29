enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
    }
}