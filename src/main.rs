mod http;
mod ssl;
mod file;
mod components;

use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configs = file::read_configs();
    let shared_configs = Arc::new(Mutex::new(configs));
    let listener_http = TcpListener::bind("0.0.0.0:80")?;
    let listener_https = TcpListener::bind("0.0.0.0:443")?;
    println!("Reverse proxy started on port 80");
    println!("Reverse proxy started on port 443");

    loop {
        //let (client_stream, _) = listener_http.accept()?;
        //let configs = shared_configs.clone();
        //tokio::spawn(async move {
        //    http::handle_client(configs, TcpStream::from_std(client_stream).unwrap()).await.unwrap();
        //});

        let (client_stream, _) = listener_https.accept()?;
        let configs = shared_configs.clone();
        tokio::spawn(async move {
            ssl::handle_client(configs, TcpStream::from_std(client_stream).unwrap()).await.unwrap();
        });
    }
}