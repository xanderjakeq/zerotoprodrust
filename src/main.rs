use std::net::TcpListener;

use rust_server::run;

#[tokio::main]
async fn main () -> std::io::Result<()> {
    let listener = TcpListener::bind("http://127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}
