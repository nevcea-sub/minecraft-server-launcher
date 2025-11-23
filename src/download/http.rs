use std::sync::OnceLock;
use reqwest::blocking::{Client, Response};
use anyhow::{Context, Result};
use log::warn;

use crate::constants::ERROR_PREVIEW_LENGTH;

static HTTP_CLIENT: OnceLock<Result<Client, String>> = OnceLock::new();

fn init_client() -> Result<Client, String> {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {e}"))
}

pub fn get_client() -> Result<&'static Client> {
    HTTP_CLIENT
        .get_or_init(init_client)
        .as_ref()
        .map_err(|e| anyhow::anyhow!("{e}"))
}

pub fn check_api_response(response: Response, context: &str) -> Result<String> {
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text()
            .unwrap_or_else(|_| format!("Failed to read error response body (status: {status})"));
        let error_preview: String = error_text.chars().take(200).collect();
        anyhow::bail!(
            "API returned status {status} for {context}: {error_preview}"
        );
    }
    response.text().with_context(|| format!("Failed to read {context} response body"))
}

pub fn parse_json_with_error_handling<T: serde::de::DeserializeOwned>(
    text: &str,
    context: &str,
) -> Result<T> {
    serde_json::from_str(text).with_context(|| {
        let preview: String = text.chars().take(ERROR_PREVIEW_LENGTH).collect();
        warn!("Failed to parse {context} JSON. Response: {preview}");
        format!("Failed to parse {context} JSON. API may have changed.")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_client() {
        let client1 = get_client().unwrap();
        let client2 = get_client().unwrap();
        assert!(std::ptr::eq(client1, client2));
    }

    #[test]
    fn test_client_creation() {
        let _client = get_client().unwrap();
    }

    #[test]
    fn test_parse_json_with_error_handling_valid() {
        let json = r#"{"versions": ["1.21.1"]}"#;
        let result: Result<serde_json::Value, _> = parse_json_with_error_handling(json, "test");
        assert!(result.is_ok());
    }
}
