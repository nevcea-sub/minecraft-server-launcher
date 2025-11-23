use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use anyhow::{Context, Result};
use log::warn;
use zip::ZipArchive;
use sha2::{Sha256, Digest};

use crate::constants::{MIN_JAR_FILE_SIZE, ZIP_MAGIC_PK1, ZIP_MAGIC_PK2, ZIP_MAGIC_ARRAY_SIZE, OPTIMAL_CHECKSUM_BUFFER_SIZE};

pub fn validate_jar_file(jar_path: &Path) -> Result<()> {
    if !jar_path.exists() {
        anyhow::bail!("JAR file does not exist: {}", jar_path.display());
    }

    let metadata = std::fs::metadata(jar_path)
        .with_context(|| format!("Failed to read metadata for {}", jar_path.display()))?;
    
    if metadata.len() == 0 {
        anyhow::bail!("JAR file is empty: {}", jar_path.display());
    }

    if metadata.len() < MIN_JAR_FILE_SIZE {
        anyhow::bail!("JAR file is too small to be valid ({} bytes): {}", metadata.len(), jar_path.display());
    }

    let mut file = File::open(jar_path)
        .with_context(|| format!("Failed to open JAR file: {}", jar_path.display()))?;

    let mut magic = [0u8; ZIP_MAGIC_ARRAY_SIZE];
    file.read_exact(&mut magic)
        .with_context(|| format!("Failed to read magic number from {}", jar_path.display()))?;

    if magic[0] != ZIP_MAGIC_PK1 || magic[1] != ZIP_MAGIC_PK2 {
        anyhow::bail!(
            "Invalid JAR file: missing ZIP magic number (expected PK, found {:02X}{:02X}): {}",
            magic[0], magic[1], jar_path.display()
        );
    }

    file.seek(std::io::SeekFrom::Start(0))
        .with_context(|| format!("Failed to seek to start of {}", jar_path.display()))?;

    match ZipArchive::new(&mut file) {
        Ok(mut archive) => {
            if archive.is_empty() {
                anyhow::bail!("JAR file contains no entries: {}", jar_path.display());
            }

            let has_manifest = archive.by_name("META-INF/MANIFEST.MF").is_ok();
            if !has_manifest {
                warn!("JAR file missing META-INF/MANIFEST.MF (may still be valid): {}", jar_path.display());
            }

            Ok(())
        }
        Err(e) => {
            anyhow::bail!(
                "Failed to parse JAR file as ZIP archive: {e} (file: {})",
                jar_path.display()
            );
        }
    }
}

pub fn validate_jar_and_calculate_checksum(jar_path: &Path) -> Result<[u8; 32]> {
    if !jar_path.exists() {
        anyhow::bail!("JAR file does not exist: {}", jar_path.display());
    }

    let metadata = std::fs::metadata(jar_path)
        .with_context(|| format!("Failed to read metadata for {}", jar_path.display()))?;
    
    if metadata.len() == 0 {
        anyhow::bail!("JAR file is empty: {}", jar_path.display());
    }

    if metadata.len() < MIN_JAR_FILE_SIZE {
        anyhow::bail!("JAR file is too small to be valid ({} bytes): {}", metadata.len(), jar_path.display());
    }

    let mut file = File::open(jar_path)
        .with_context(|| format!("Failed to open JAR file: {}", jar_path.display()))?;

    let mut magic = [0u8; ZIP_MAGIC_ARRAY_SIZE];
    file.read_exact(&mut magic)
        .with_context(|| format!("Failed to read magic number from {}", jar_path.display()))?;

    if magic[0] != ZIP_MAGIC_PK1 || magic[1] != ZIP_MAGIC_PK2 {
        anyhow::bail!(
            "Invalid JAR file: missing ZIP magic number (expected PK, found {:02X}{:02X}): {}",
            magic[0], magic[1], jar_path.display()
        );
    }

    file.seek(std::io::SeekFrom::Start(0))
        .with_context(|| format!("Failed to seek to start of {}", jar_path.display()))?;

    let mut archive = ZipArchive::new(&mut file)
        .map_err(|e| anyhow::anyhow!("Failed to parse JAR file as ZIP archive: {e} (file: {})", jar_path.display()))?;
    
    if archive.is_empty() {
        anyhow::bail!("JAR file contains no entries: {}", jar_path.display());
    }

    let has_manifest = archive.by_name("META-INF/MANIFEST.MF").is_ok();
    if !has_manifest {
        warn!("JAR file missing META-INF/MANIFEST.MF (may still be valid): {}", jar_path.display());
    }

    file.seek(std::io::SeekFrom::Start(0))
        .with_context(|| format!("Failed to seek to start for checksum: {}", jar_path.display()))?;

    let file_size = metadata.len();
    let mut hasher = Sha256::new();
    
    if file_size < 64 * 1024 {
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = file.read(&mut buffer)
                .with_context(|| format!("Failed to read file for checksum: {}", jar_path.display()))?;
            
            if bytes_read == 0 {
                break;
            }
            
            hasher.update(&buffer[..bytes_read]);
        }
    } else {
        let mut buffer = vec![0u8; OPTIMAL_CHECKSUM_BUFFER_SIZE];
        
        loop {
            let bytes_read = file.read(&mut buffer)
                .with_context(|| format!("Failed to read file for checksum: {}", jar_path.display()))?;
            
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use std::io::Write;

    #[test]
    fn test_validate_jar_file_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let jar_path = temp_dir.path().join("nonexistent.jar");
        
        let result = validate_jar_file(&jar_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_validate_jar_file_empty() {
        let temp_dir = TempDir::new().unwrap();
        let jar_path = temp_dir.path().join("empty.jar");
        fs::write(&jar_path, b"").unwrap();
        
        let result = validate_jar_file(&jar_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_validate_jar_file_invalid_magic() {
        let temp_dir = TempDir::new().unwrap();
        let jar_path = temp_dir.path().join("invalid.jar");
        let mut file = File::create(&jar_path).unwrap();
        file.write_all(&[0u8; 100]).unwrap();
        file.write_all(b"INVALID").unwrap();
        
        let result = validate_jar_file(&jar_path);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("magic number") || error_msg.contains("ZIP"), 
                "Error message should contain 'magic number' or 'ZIP', got: {error_msg}");
    }

    #[test]
    fn test_validate_jar_file_too_small() {
        let temp_dir = TempDir::new().unwrap();
        let jar_path = temp_dir.path().join("small.jar");
        let mut file = File::create(&jar_path).unwrap();
        file.write_all(b"PK\x03\x04").unwrap();
        
        let result = validate_jar_file(&jar_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too small"));
    }
}
