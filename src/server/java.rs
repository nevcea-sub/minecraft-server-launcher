use std::process::{Command, Stdio};
use anyhow::{Context, Result};
use log::warn;

use crate::constants::{JAVA_CMD, JAVA_VERSION_ARG, MIN_JAVA_VERSION};

const VERSION_UNKNOWN: &str = "unknown";
const VERSION_KEYWORD: &str = "version";

pub fn check_java() -> Result<String> {
    let output = Command::new(JAVA_CMD)
        .arg(JAVA_VERSION_ARG)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .context("Java is not installed or not in PATH. Please install Java 17 or higher.")?;

    let version_output = String::from_utf8_lossy(&output.stderr);
    let version = extract_java_version(&version_output);

    if version != VERSION_UNKNOWN {
        if let Some(version_str) = version.split('.').next() {
            if let Ok(version_num) = version_str.parse::<u32>() {
                if version_num < MIN_JAVA_VERSION {
                    warn!(
                        "Java version {} detected. Minecraft 1.18+ requires Java {} or higher. The server may not start correctly.",
                        version, MIN_JAVA_VERSION
                    );
                }
            }
        }
    }

    Ok(version)
}

fn extract_java_version(output: &str) -> String {
    for line in output.lines() {
        if let Some(version) = extract_version_from_line(line) {
            return version;
        }
    }
    VERSION_UNKNOWN.to_string()
}

fn extract_version_from_line(line: &str) -> Option<String> {
    if !line.to_lowercase().contains(VERSION_KEYWORD) {
        return None;
    }
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    for (i, part) in parts.iter().enumerate() {
        if part.to_lowercase().contains(VERSION_KEYWORD) && i + 1 < parts.len() {
            if let Some(version) = extract_version_from_part(parts[i + 1]) {
                return Some(version);
            }
        }
    }
    
    for part in parts {
        if let Some(version) = extract_version_from_part(part) {
            return Some(version);
        }
    }
    
    None
}

fn extract_version_from_part(part: &str) -> Option<String> {
    let cleaned: String = part
        .trim_matches('"')
        .trim_matches('\'')
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    
    if cleaned.len() >= 2 && cleaned.chars().any(|c| c.is_ascii_digit()) {
        if let Some(first_dot) = cleaned.find('.') {
            if first_dot > 0 {
                return Some(cleaned);
            }
        } else if !cleaned.is_empty() {
            return Some(cleaned);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version_from_line() {
        assert_eq!(
            extract_version_from_line("openjdk version \"17.0.1\" 2021-10-19"),
            Some("17.0.1".to_string())
        );
    }
}
