mod http;
mod ssl;
mod file;
mod components;
mod dashboard;

use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct BColors {
    pub header: String,
    pub blue: String,
    pub cyan: String,
    pub cyan_green: String,
    pub warning: String,
    pub fail: String,
    pub endc: String,
    pub bold: String,
    pub underline: String,
}

impl BColors {
    fn new() -> Self {
        BColors {
            header: "\x1b[95m".to_string(),
            blue: "\x1b[94m".to_string(),
            cyan: "\x1b[96m".to_string(),
            cyan_green: "\x1b[92m".to_string(),
            warning: "\x1b[93m".to_string(),
            fail: "\x1b[91m".to_string(),
            endc: "\x1b[0m".to_string(),
            bold: "\x1b[1m".to_string(),
            underline: "\x1b[4m".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let colors = BColors::new();

    let configs = file::read_configs();
    let shared_configs = Arc::new(Mutex::new(configs));

    let listener_http = TcpListener::bind("0.0.0.0:80").await?;
    let listener_https = TcpListener::bind("0.0.0.0:443").await?;

    println!("{}[ARCTICARCH]{} Reverse proxy started on port 80", colors.blue, colors.endc);
    println!("{}[ARCTICARCH]{} Reverse proxy started on port 443", colors.blue, colors.endc);

    let listener_dashboard = if !env::var("USERNAME").is_err() && !env::var("PASSWORD").is_err() && !env::var("USERNAME").unwrap().is_empty() && !env::var("PASSWORD").unwrap().is_empty() {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;
        println!("{}[ARCTICARCH]{} Dashboard started on port 8080", colors.blue, colors.endc);
        Some(listener)
    } else {
        None
    };

    loop {
        tokio::select! {
            Ok((client_stream, _)) = listener_http.accept() => {
                let configs = shared_configs.clone();
                let colors_clone = colors.clone();
                tokio::spawn(async move {
                    http::handle_client(configs, colors_clone, client_stream).await.unwrap();
                });
            },
            Ok((client_stream, _)) = listener_https.accept() => {
                let configs = shared_configs.clone();
                let colors_clone = colors.clone();
                tokio::spawn(async move {
                    ssl::handle_client(configs, colors_clone, client_stream).await.unwrap();
                });
            },
            Ok((client_stream, _)) = listener_dashboard.as_ref().unwrap().accept() => {
                tokio::spawn(async move {
                    dashboard::handle_request(client_stream).await.unwrap();
                });
            },
        }
    }
}
