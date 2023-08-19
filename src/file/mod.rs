use std::fs;

pub fn parse_config(file_content: &str) -> Option<(String, bool, String)> {
    let lines: Vec<&str> = file_content.lines().collect();
    if lines.len() != 3 {
        return None;
    }
    let domain = lines[0].trim().strip_prefix("domain: ")?;
    let ssl = lines[1].trim().strip_prefix("ssl: ").and_then(|v| v.parse().ok())?;
    let location = lines[2].trim().strip_prefix("location: ")?;
    Some((domain.to_string(), ssl, location.to_string()))
}

pub fn read_configs() -> Vec<(String, bool, String)> {
    let mut configs = Vec::new();
    for entry in fs::read_dir("./domains").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("conf") {
            let content = fs::read_to_string(&path).unwrap();
            if let Some(config) = parse_config(&content) {
                configs.push(config);
            }
        }
    }
    configs
}
