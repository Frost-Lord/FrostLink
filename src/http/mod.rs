use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::time::{Instant};
use std::fs::read_to_string;
use tokio::io;

pub async fn handle_client(configs: Arc<Mutex<Vec<(String, bool, String, String, String)>>>, mut client_stream: TcpStream) -> std::io::Result<()> {
    let start_time = Instant::now();

    let mut buffer = vec![0; 1024];
    let size = client_stream.read(&mut buffer).await?;
    let request_str = String::from_utf8_lossy(&buffer[0..size]);
    let request_path = request_str.split_whitespace().nth(1).unwrap_or("/").to_string();

    let domain_and_location = {
        let configs = configs.lock().await;
        let found_domain = configs.iter().find(|(domain, _, _, _, _)| {
            //println!("In request: {}", request_str);
            request_str.contains(&format!("Host: {}", domain))
        });
        found_domain.cloned()
    };

    if let Some((domain, _, location, _, _)) = domain_and_location {
        if let Ok(mut local_stream) = TcpStream::connect(&location).await {
            if local_stream.write(&buffer[0..size]).await.is_ok() {
                if io::copy(&mut local_stream, &mut client_stream).await.is_ok() {
                    let ip = client_stream.peer_addr().unwrap().ip();
                    let elapsed_time = start_time.elapsed();
                    println!("Time taken: {:?} : {} : {} : {} : SSL: False", elapsed_time, ip, domain, request_path);
                } else {
                    eprintln!("Failed to send response to client");
                }
            } else {
                eprintln!("Failed to send request to {}", location);
            }
        } else {
            eprintln!("Could not connect to {}", location);
        }
    } else {
        let default_file_path = "./default/index.html";
        match read_to_string(default_file_path) {
            Ok(contents) => {
                let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", contents);
                client_stream.write_all(response.as_bytes()).await?;
            }
            Err(_) => {
                let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n<html><head><title>Not Found</title></head><body><h1>404 - File Not Found</h1></body></html>";
                client_stream.write_all(response.as_bytes()).await?;
                eprintln!("Failed to read the default index.html file from {}", default_file_path);
            }
        }
    }

    Ok(())
}