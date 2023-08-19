mod http;
mod ssl;
mod file;

use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configs = file::read_configs();
    let listener_http = TcpListener::bind("0.0.0.0:80")?;
    let listener_https = TcpListener::bind("0.0.0.0:443")?;
    println!("Reverse proxy started on port 80");
    println!("Reverse proxy started on port 443");

    loop {
        let (client_stream, _) = listener_http.accept()?;
        let configs = Arc::new(Mutex::new(configs.clone()));
        tokio::spawn(async move {
            http::handle_client(configs, TcpStream::from_std(client_stream).unwrap()).await.unwrap();
        });
    }
}
