use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use anyhow::{Context, Result};
use log::warn;
use zip::ZipArchive;

pub fn validate_jar_file(jar_path: &Path) -> Result<()> {
    if !jar_path.exists() {
        anyhow::bail!("JAR file does not exist: {:?}", jar_path);
    }

    let metadata = std::fs::metadata(jar_path)
        .with_context(|| format!("Failed to read metadata for {:?}", jar_path))?;
    
    if metadata.len() == 0 {
        anyhow::bail!("JAR file is empty: {:?}", jar_path);
    }

    if metadata.len() < 22 {
        anyhow::bail!("JAR file is too small to be valid ({} bytes): {:?}", metadata.len(), jar_path);
    }

    let mut file = File::open(jar_path)
        .with_context(|| format!("Failed to open JAR file: {:?}", jar_path))?;

    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)
        .with_context(|| format!("Failed to read magic number from {:?}", jar_path))?;

    if magic[0] != 0x50 || magic[1] != 0x4B {
        anyhow::bail!(
            "Invalid JAR file: missing ZIP magic number (expected PK, found {:02X}{:02X}): {:?}",
            magic[0], magic[1], jar_path
        );
    }

    file.seek(std::io::SeekFrom::Start(0))
        .with_context(|| format!("Failed to seek to start of {:?}", jar_path))?;

    match ZipArchive::new(&mut file) {
        Ok(mut archive) => {
            if archive.len() == 0 {
                anyhow::bail!("JAR file contains no entries: {:?}", jar_path);
            }

            let has_manifest = archive.by_name("META-INF/MANIFEST.MF").is_ok();
            if !has_manifest {
                warn!("JAR file missing META-INF/MANIFEST.MF (may still be valid): {:?}", jar_path);
            }

            Ok(())
        }
        Err(e) => {
            anyhow::bail!(
                "Failed to parse JAR file as ZIP archive: {} (file: {:?})",
                e, jar_path
            );
        }
    }
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
        file.write_all(&vec![0u8; 100]).unwrap();
        file.write_all(b"INVALID").unwrap();
        
        let result = validate_jar_file(&jar_path);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("magic number") || error_msg.contains("ZIP"), 
                "Error message should contain 'magic number' or 'ZIP', got: {}", error_msg);
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

