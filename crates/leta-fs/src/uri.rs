use std::path::{Path, PathBuf};

pub fn path_to_uri(path: &Path) -> String {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let path_str = path.to_string_lossy();
    let encoded = encode_uri_path(&path_str);
    format!("file://{}", encoded)
}

fn encode_uri_path(path: &str) -> String {
    let mut result = String::with_capacity(path.len() * 2);
    for c in path.chars() {
        match c {
            '[' => result.push_str("%5B"),
            ']' => result.push_str("%5D"),
            ' ' => result.push_str("%20"),
            '#' => result.push_str("%23"),
            '?' => result.push_str("%3F"),
            '%' => result.push_str("%25"),
            _ => result.push(c),
        }
    }
    result
}

pub fn uri_to_path(uri: &str) -> PathBuf {
    if let Some(path) = uri.strip_prefix("file://") {
        let decoded = decode_uri_path(path);
        PathBuf::from(decoded)
    } else {
        PathBuf::from(uri)
    }
}

fn decode_uri_path(path: &str) -> String {
    let mut result = String::with_capacity(path.len());
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if hex.len() == 2 {
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                    continue;
                }
            }
            result.push('%');
            result.push_str(&hex);
        } else {
            result.push(c);
        }
    }
    result
}
