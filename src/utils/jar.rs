use std::path::Path;
use anyhow::{Context, Result};
use log::{info, warn};

use crate::api::{API_BASE, PaperBuildsResponse, PaperDownload, PaperProject};
use crate::constants::{CURRENT_DIR, DEFAULT_VERSION, JAR_EXTENSION, JAR_PREFIX};
use crate::download::download_file;
use crate::download::http::{check_api_response, get_client, parse_json_with_error_handling};
use crate::utils::validation::validate_jar_file;

pub fn find_jar_file() -> Result<Option<String>> {
    for entry in std::fs::read_dir(CURRENT_DIR).context("Failed to read current directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(JAR_PREFIX) && name.ends_with(JAR_EXTENSION) {
                    if let Err(e) = validate_jar_file(&path) {
                        warn!("Found JAR file '{}' but validation failed: {}. Skipping.", name, e);
                        continue;
                    }
                    return Ok(Some(name.to_string()));
                }
            }
        }
    }
    Ok(None)
}

pub fn download_jar(version: &str) -> Result<String> {
    let version_str = if version == DEFAULT_VERSION {
        let response = get_client()?
            .get(API_BASE)
            .send()
            .context("Failed to fetch project info from API")?;
        
        let text = check_api_response(response, "project info")?;
        let project: PaperProject = parse_json_with_error_handling(&text, "project")?;
        
        project.versions
            .last()
            .ok_or_else(|| anyhow::anyhow!("No versions found"))?
            .clone()
    } else {
        version.to_string()
    };

    let builds_url = format!("{}/versions/{}/builds", API_BASE, version_str);
    let response = get_client()?
        .get(&builds_url)
        .send()
        .with_context(|| format!("Failed to fetch builds for version {}", version_str))?;
    
    let text = check_api_response(response, "builds")?;
    let builds_response: PaperBuildsResponse = parse_json_with_error_handling(&text, "builds")?;
    
    let latest_build = builds_response.builds.last()
        .ok_or_else(|| anyhow::anyhow!("No builds found for version {}", version_str))?;

    let download_info_url = format!(
        "{}/versions/{}/builds/{}",
        API_BASE, version_str, latest_build.build
    );
    
    let response = get_client()?
        .get(&download_info_url)
        .send()
        .with_context(|| format!("Failed to fetch download info for build {}", latest_build.build))?;
    
    let text = check_api_response(response, "download info")?;
    let download: PaperDownload = parse_json_with_error_handling(&text, "download")?;

    let jar_name = &download.downloads.application.name;
    let jar_path = Path::new(jar_name);
    
    if jar_path.exists() {
        validate_jar_file(jar_path)
            .with_context(|| format!("Existing JAR file failed validation: {}", jar_name))?;
        info!("Validated existing JAR file: {}", jar_name);
        return Ok(jar_name.clone());
    }

    let download_url = format!(
        "{}/versions/{}/builds/{}/downloads/{}",
        API_BASE, version_str, latest_build.build, jar_name
    );

    download_file(&download_url, jar_path, jar_name)?;
    
    validate_jar_file(jar_path)
        .with_context(|| format!("Downloaded JAR file failed validation: {}", jar_name))?;
    info!("Validated downloaded JAR file: {}", jar_name);
    
    Ok(jar_name.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jar_prefix_and_extension() {
        assert_eq!(JAR_PREFIX, "paper-");
        assert_eq!(JAR_EXTENSION, ".jar");
    }
}
 