use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};

use crate::constants::{DOWNLOAD_BUFFER_SIZE, PROGRESS_UPDATE_INTERVAL};
use crate::download::http::get_client;

const PROGRESS_TEMPLATE: &str = "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
const PROGRESS_CHARS: &str = "#>-";

pub fn download_file(url: &str, file_path: &Path, _display_name: &str) -> Result<()> {
    if file_path.exists() {
        return Ok(());
    }

    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(PROGRESS_TEMPLATE)
            .unwrap()
            .progress_chars(PROGRESS_CHARS),
    );

    let mut response = get_client()?
        .get(url)
        .send()
        .with_context(|| format!("Failed to download from {url}"))?;
    
    pb.set_length(response.content_length().unwrap_or(0));

    let file = File::create(file_path)
        .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
    
    let mut writer = BufWriter::with_capacity(DOWNLOAD_BUFFER_SIZE, file);
    let mut downloaded = 0u64;
    let mut buffer = vec![0u8; DOWNLOAD_BUFFER_SIZE];
    let mut last_update = 0u64;

    loop {
        let bytes_read = response.read(&mut buffer)
            .context("Failed to read from response")?;
        
        if bytes_read == 0 {
            break;
        }

        writer.write_all(&buffer[..bytes_read])
            .context("Failed to write to file")?;
        
        downloaded += bytes_read as u64;
        
        if downloaded - last_update >= PROGRESS_UPDATE_INTERVAL {
            pb.set_position(downloaded);
            last_update = downloaded;
        }
    }

    pb.set_position(downloaded);
    writer.flush().context("Failed to flush file")?;
    pb.finish();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_download_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test").unwrap();

        let result = download_file("http://example.com", &file_path, "test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_download_file_invalid_url() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let result = download_file("http://invalid-url-that-does-not-exist-12345.com/file", &file_path, "test.txt");
        assert!(result.is_err());
    }
}

