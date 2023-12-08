use std::io::Result;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::dashboard::SESSIONS;

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

pub async fn handle_api_request(configs: Arc<Mutex<Vec<(String, bool, String, String, String)>>>, path: &str, buffer: &[u8]) -> Result<String> {
    match path {
        "/api/login" => {
            let json = extract_json(buffer).await;

            let username = json.get("username").and_then(|u| u.as_str()).unwrap_or("");
            let password = json.get("password").and_then(|p| p.as_str()).unwrap_or("");

            let valid = username == "ewen2076@gmail.com" && password == "Fighting35a";
            if valid {
                let session_id = Uuid::new_v4().to_string();
                let mut sessions = SESSIONS.lock().await;
                sessions.insert(session_id.clone(), username.to_string());
            
                let mut data = HashMap::new();
                data.insert("valid", Value::Bool(true));
                data.insert("session_id", Value::String(session_id));
                Ok(construct_json_response(data))
                
            } else {
                let mut data = HashMap::new();
                data.insert("valid", Value::Bool(false));
                Ok(construct_json_response(data))
            }
        },
        "/api/proxys" => {
            let locked_configs = configs.lock().await;
            let configs_json: Vec<_> = locked_configs.iter().map(|(domain, ssl, host, pubkey, privkey)| {
                let mut config_map = serde_json::Map::new();
                config_map.insert("domain".to_string(), serde_json::Value::String(domain.clone()));
                config_map.insert("host".to_string(), serde_json::Value::String(host.clone()));
                config_map.insert("SSL".to_string(), serde_json::Value::Bool(*ssl));
                config_map.insert("pubkey".to_string(), serde_json::Value::String(pubkey.clone()));
                config_map.insert("privkey".to_string(), serde_json::Value::String(privkey.clone()));
                serde_json::Value::Object(config_map)
            }).collect();
        
            let mut data = HashMap::new();
            data.insert("configs", serde_json::Value::Array(configs_json));
        
            Ok(construct_json_response(data))
        },        
        _ => Ok("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string())
    }
}