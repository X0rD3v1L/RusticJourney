use server::create_server;

mod command;
mod database;
mod server;

#[tokio::main]
async fn main() {
    let add = "127.0.0.1:3002";
    eprintln!("Server will start at {:?}", add);
    create_server(add).await;
}
