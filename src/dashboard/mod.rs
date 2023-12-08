use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::fs;
use std::io::Result;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

mod api;

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub async fn handle_request(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).await?;

    let request_line = std::str::from_utf8(&buffer).unwrap_or("");
    let request_path = request_line
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("");

    if request_path.starts_with("/api/") {
        let response = api::handle_api_request(request_path, &buffer).await?;
        stream.write_all(response.as_bytes()).await?;
    } else {
        serve_html_file(&mut stream, request_path, &buffer).await?;
    }

    stream.flush().await
}

async fn serve_html_file(stream: &mut TcpStream, request_path: &str, buffer: &[u8]) -> Result<()> {
    let file_path = if request_path == "/" {
        "./default/login.html".to_string()
    } else {
        format!("./default{}.html", request_path)
    };

    let response = if let Some(contents) = cache_get(&file_path).await {
        contents
    } else if request_path.starts_with("/styles/") {
        serve_css_file(&buffer).await?
    } else {
        cache_get("./default/404.html").await.unwrap_or_else(|| String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"))
    };

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await
}

async fn cache_get(path: &str) -> Option<String> {
    let mut cache = CACHE.lock().unwrap();
    if cache.contains_key(path) {
        Some(cache.get(path).unwrap().clone())
    } else {
        if Path::new(path).exists() {
            let contents = fs::read_to_string(path).ok()?;
            cache.insert(path.to_string(), format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents));
            Some(cache.get(path).unwrap().clone())
        } else {
            None
        }
    }
}

async fn serve_css_file(buffer: &[u8]) -> Result<String> {
    let request_line = std::str::from_utf8(buffer).unwrap_or("");
    let css_file_path = request_line
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .map(|path| format!("./default{}", path))
        .unwrap_or_default();

    if Path::new(&css_file_path).exists() {
        let contents = fs::read_to_string(css_file_path)?;
        Ok(format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        ))
    } else {
        Ok(String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n"))
    }
}
