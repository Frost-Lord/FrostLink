use std::io::Result;
use serde_json::Value;
use std::collections::HashMap;

async fn extract_json(buffer: &[u8]) -> Value {
    let buffer_string = std::str::from_utf8(buffer).unwrap_or("");
    let content_length = buffer_string
        .lines()
        .find(|line| line.starts_with("Content-Length:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|num| num.parse::<usize>().ok())
        .unwrap_or(0);

    let json_start_index = buffer_string.find("\r\n\r\n").unwrap_or(0) + 4;
    let json_body = &buffer_string[json_start_index..json_start_index + content_length];
    match serde_json::from_str(json_body) {
        Ok(val) => val,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            serde_json::json!({})
        }
    }
}

fn construct_json_response(data: HashMap<&str, Value>) -> String {
    let json_body = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());
    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", json_body)
}


pub async fn handle_api_request(path: &str, buffer: &[u8]) -> Result<String> {
    match path {
        "/api/login" => {
            let json = extract_json(buffer).await;

            let username = json.get("username").and_then(|u| u.as_str()).unwrap_or("");
            let password = json.get("password").and_then(|p| p.as_str()).unwrap_or("");

            let valid = username == "root" && password == "password";
            let mut data = HashMap::new();
            data.insert("valid", Value::Bool(valid));
            Ok(construct_json_response(data))
        },
        "/api/stats" => {
            Ok("Response for stats".to_string())
        },
        _ => Ok("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string())
    }
}