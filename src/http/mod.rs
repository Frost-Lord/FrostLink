use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::time::{Instant};
use tokio::io;

pub async fn handle_client(configs: Arc<Mutex<Vec<(String, bool, String)>>>, mut client_stream: TcpStream) -> std::io::Result<()> {
    let start_time = Instant::now();

    let mut buffer = vec![0; 1024];
    let size = client_stream.read(&mut buffer).await?;
    let request_str = String::from_utf8_lossy(&buffer[0..size]);
    let request_path = request_str.split_whitespace().nth(1).unwrap_or("/").to_string();

    let domain_and_location = {
        let configs = configs.lock().await;
        configs.iter().find(|(domain, _, _)| request_str.contains(&format!("Host: {}", domain))).cloned()
    };

    if let Some((domain, _, location)) = domain_and_location {
        if let Ok(mut local_stream) = TcpStream::connect(&location).await {
            if local_stream.write(&buffer[0..size]).await.is_ok() {
                if io::copy(&mut local_stream, &mut client_stream).await.is_ok() {
                    let ip = client_stream.peer_addr().unwrap().ip();
                    let elapsed_time = start_time.elapsed();
                    println!("Time taken: {:?} : {} : {} : {}", elapsed_time, ip, domain, request_path);
                } else {
                    eprintln!("Failed to send response to client");
                }
            } else {
                eprintln!("Failed to send request to {}", location);
            }
        } else {
            eprintln!("Could not connect to {}", location);
        }
    }

    Ok(())
}