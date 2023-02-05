use std::net::TcpListener;
use dotenv::dotenv;
use std::env;

use rust_server::run;

#[tokio::main]
async fn main () -> std::io::Result<()> {
    dotenv().ok();
    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        //"127.0.0.1:{}",
        env::var("PORT").unwrap_or("0".to_owned())
    ))
    .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    println!("listening in port: {}", port);

    run(listener)?.await
}
