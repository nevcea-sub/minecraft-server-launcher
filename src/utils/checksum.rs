use std::fs::File;
use std::io::Read;
use std::path::Path;
use anyhow::{Context, Result};
use sha2::{Sha256, Digest};
use hex;

use crate::constants::OPTIMAL_CHECKSUM_BUFFER_SIZE;

pub fn calculate_file_sha256_bytes(file_path: &Path) -> Result<[u8; 32]> {
    let mut file = File::open(file_path)
        .with_context(|| format!("Failed to open file for checksum calculation: {}", file_path.display()))?;
    
    let file_size = file.metadata()
        .with_context(|| format!("Failed to read metadata for {}", file_path.display()))?
        .len();
    
    let mut hasher = Sha256::new();
    
    if file_size < 64 * 1024 {
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = file.read(&mut buffer)
                .with_context(|| format!("Failed to read file for checksum: {}", file_path.display()))?;
            
            if bytes_read == 0 {
                break;
            }
            
            hasher.update(&buffer[..bytes_read]);
        }
    } else {
        let mut buffer = vec![0u8; OPTIMAL_CHECKSUM_BUFFER_SIZE];
        
        loop {
            let bytes_read = file.read(&mut buffer)
                .with_context(|| format!("Failed to read file for checksum: {}", file_path.display()))?;
            
            if bytes_read == 0 {
                break;
            }
            
            hasher.update(&buffer[..bytes_read]);
        }
    }
    
    let hash = hasher.finalize();
    let mut result = [0u8; 32];
    result.copy_from_slice(hash.as_slice());
    Ok(result)
}

pub fn calculate_file_sha256(file_path: &Path) -> Result<String> {
    let bytes = calculate_file_sha256_bytes(file_path)?;
    Ok(hex::encode(bytes))
}

pub fn validate_file_checksum(file_path: &Path, expected_hash: Option<&str>) -> Result<()> {
    let Some(expected) = expected_hash else {
        return Ok(());
    };
    
    let expected_trimmed = expected.trim();
    let expected_bytes = hex::decode(expected_trimmed)
        .ok()
        .filter(|bytes| bytes.len() == 32);
    
    if let Some(expected_bytes) = expected_bytes {
        let actual_bytes = calculate_file_sha256_bytes(file_path)?;
        if actual_bytes != expected_bytes.as_slice() {
            let actual = hex::encode(actual_bytes);
            anyhow::bail!(
                "Checksum mismatch for file: {}\nExpected: {}\nActual: {}",
                file_path.display(),
                expected,
                actual
            );
        }
    } else {
        let actual = calculate_file_sha256(file_path)?;
        if !actual.eq_ignore_ascii_case(expected_trimmed) {
            anyhow::bail!(
                "Checksum mismatch for file: {}\nExpected: {}\nActual: {}",
                file_path.display(),
                expected,
                actual
            );
        }
    }
    
    Ok(())
}

pub fn load_checksum_from_file(checksum_path: &Path) -> Result<Option<String>> {
    if !checksum_path.exists() {
        return Ok(None);
    }
    
    let content = std::fs::read_to_string(checksum_path)
        .with_context(|| format!("Failed to read checksum file: {}", checksum_path.display()))?;
    
    Ok(content
        .lines()
        .next()
        .and_then(|line| {
            let trimmed = line.trim();
            (trimmed.len() == 64).then_some(trimmed.to_string())
        }))
}

pub fn save_checksum_to_file(checksum_path: &Path, checksum: &str) -> Result<()> {
    std::fs::write(checksum_path, checksum)
        .with_context(|| format!("Failed to write checksum file: {}", checksum_path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_calculate_file_sha256() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();
        
        let hash = calculate_file_sha256(&file_path).unwrap();
        assert_eq!(hash, "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_validate_file_checksum_valid() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();
        
        let expected = "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72";
        assert!(validate_file_checksum(&file_path, Some(expected)).is_ok());
    }

    #[test]
    fn test_validate_file_checksum_invalid() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "different content").unwrap();
        
        let expected = "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72";
        assert!(validate_file_checksum(&file_path, Some(expected)).is_err());
    }

    #[test]
    fn test_validate_file_checksum_no_expected() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();
        
        assert!(validate_file_checksum(&file_path, None).is_ok());
    }
}

