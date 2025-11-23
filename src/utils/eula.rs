use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

use crate::constants::{EULA_CONTENT, EULA_FILE};

pub fn handle_eula() -> Result<()> {
    let eula_path = Path::new(EULA_FILE);
    
    if eula_path.exists() {
        let content = fs::read_to_string(eula_path)
            .context("Failed to read eula.txt")?;
        
        if content.lines().any(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with('#') && trimmed.to_lowercase().contains("eula=true")
        }) {
            return Ok(());
        }
    }

    fs::write(eula_path, EULA_CONTENT)
        .context("Failed to write eula.txt")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eula_content_contains_true() {
        assert!(EULA_CONTENT.contains("eula=true"));
    }
}

