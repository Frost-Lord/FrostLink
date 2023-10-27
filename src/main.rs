mod http;
mod ssl;
mod file;
mod components;

use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;

#[derive(Clone)]
pub struct BColors {
    pub header: &'static str,
    pub blue: &'static str,
    pub cyan: &'static str,
    pub cyan_green: &'static str,
    pub warning: &'static str,
    pub fail: &'static str,
    pub endc: &'static str,
    pub bold: &'static str,
    pub underline: &'static str,
}

impl BColors {
    fn new() -> Self {
        BColors {
            header: "\x1b[95m",
            blue: "\x1b[94m",
            cyan: "\x1b[96m",
            cyan_green: "\x1b[92m",
            warning: "\x1b[93m",
            fail: "\x1b[91m",
            endc: "\x1b[0m",
            bold: "\x1b[1m",
            underline: "\x1b[4m",
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let colors = BColors::new();

    let configs = file::read_configs();
    let shared_configs = Arc::new(Mutex::new(configs));
    let _listener_http = TcpListener::bind("0.0.0.0:80")?;
    let listener_https = TcpListener::bind("0.0.0.0:443")?;
    println!("{}[ARCTICARCH]{} Reverse proxy started on port 80", colors.blue, colors.endc);
    println!("{}[ARCTICARCH]{} Reverse proxy started on port 443", colors.blue, colors.endc);

    loop {
        // Uncomment for HTTP
        // let (client_stream, _) = listener_http.accept()?;
        // let configs = shared_configs.clone();
        // let colors = colors.clone();
        // tokio::spawn(async move {
        //     http::handle_client(configs, colors, TcpStream::from_std(client_stream).unwrap()).await.unwrap();
        // });

        let (client_stream, _) = listener_https.accept()?;
        let configs = shared_configs.clone();
        let colors = colors.clone();
        tokio::spawn(async move {
            ssl::handle_client(configs, colors, TcpStream::from_std(client_stream).unwrap()).await.unwrap();
        });
    }
}
