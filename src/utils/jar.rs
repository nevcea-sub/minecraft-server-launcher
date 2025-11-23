use std::path::Path;
use anyhow::{Context, Result};
use log::{info, warn};
use hex;

use crate::api::{API_BASE, PaperBuildsResponse, PaperDownload, PaperProject};
use crate::constants::{CURRENT_DIR, DEFAULT_VERSION, JAR_EXTENSION, JAR_PREFIX};
use crate::download::download_file;
use crate::download::http::{check_api_response, get_client, parse_json_with_error_handling};
use crate::utils::validation::validate_jar_file;
use crate::utils::checksum::{validate_file_checksum, load_checksum_from_file, save_checksum_to_file};

pub fn find_jar_file() -> Result<Option<String>> {
    let mut validation_count = 0;
    
    for entry in std::fs::read_dir(CURRENT_DIR).context("Failed to read current directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with(JAR_PREFIX) && name.ends_with(JAR_EXTENSION) {
                    match validate_jar_file(&path) {
                        Ok(()) => return Ok(Some(name.to_string())),
                        Err(e) => {
                            validation_count += 1;
                            warn!("JAR file '{name}' validation failed: {e}. Skipping.");
                        }
                    }
                }
            }
        }
    }
    
    if validation_count > 0 {
        warn!("Found {} JAR file(s) but all failed validation. Please check the files or download a new one.", validation_count);
    }
    
    Ok(None)
}

fn resolve_version(version: &str) -> Result<String> {
    if version == DEFAULT_VERSION {
        let response = get_client()?
            .get(API_BASE)
            .send()
            .context("Failed to fetch project info from API")?;
        
        let text = check_api_response(response, "project info")?;
        let project: PaperProject = parse_json_with_error_handling(&text, "project")?;
        
        project.versions
            .last()
            .ok_or_else(|| anyhow::anyhow!("No versions found"))
            .cloned()
    } else {
        Ok(String::from(version))
    }
}

fn fetch_latest_build(version: &str) -> Result<u32> {
    let builds_url = format!("{API_BASE}/versions/{version}/builds");
    let response = get_client()?
        .get(&builds_url)
        .send()
        .with_context(|| format!("Failed to fetch builds for version {version}"))?;
    
    let text = check_api_response(response, "builds")?;
    let builds_response: PaperBuildsResponse = parse_json_with_error_handling(&text, "builds")?;
    
    builds_response.builds
        .last()
        .ok_or_else(|| anyhow::anyhow!("No builds found for version {version}"))
        .map(|build| build.build)
}

fn fetch_download_info(version: &str, build: u32) -> Result<PaperDownload> {
    let download_info_url = format!("{API_BASE}/versions/{version}/builds/{build}");
    
    let response = get_client()?
        .get(&download_info_url)
        .send()
        .with_context(|| format!("Failed to fetch download info for build {build}"))?;
    
    let text = check_api_response(response, "download info")?;
    parse_json_with_error_handling(&text, "download")
}

fn handle_existing_jar(jar_path: &Path, jar_name: &str) -> Result<()> {
    let checksum_path = jar_path.with_extension("jar.sha256");
    match load_checksum_from_file(&checksum_path) {
        Ok(Some(expected_checksum)) => {
            validate_jar_file(jar_path)
                .with_context(|| format!("Existing JAR file failed validation: {jar_name}"))?;
            
            let expected_bytes = hex::decode(expected_checksum.trim())
                .ok()
                .filter(|bytes| bytes.len() == 32);
            
            if let Some(expected_bytes) = expected_bytes {
                let actual_bytes = crate::utils::validation::validate_jar_and_calculate_checksum(jar_path)
                    .with_context(|| format!("Failed to validate and calculate checksum for existing JAR: {jar_name}"))?;
                
                if actual_bytes != expected_bytes.as_slice() {
                    let actual = hex::encode(actual_bytes);
                    anyhow::bail!(
                        "Existing JAR file checksum validation failed: {jar_name}\nExpected: {}\nActual: {}",
                        expected_checksum,
                        actual
                    );
                }
            } else {
                validate_jar_file(jar_path)
                    .with_context(|| format!("Existing JAR file failed validation: {jar_name}"))?;
                validate_file_checksum(jar_path, Some(&expected_checksum))
                    .with_context(|| format!("Existing JAR file checksum validation failed: {jar_name}"))?;
            }
            
            info!("Validated existing JAR file checksum: {jar_name}");
        }
        _ => {
            let checksum_bytes = crate::utils::validation::validate_jar_and_calculate_checksum(jar_path)
                .with_context(|| format!("Failed to validate and calculate checksum for existing JAR: {jar_name}"))?;
            let checksum = hex::encode(checksum_bytes);
            save_checksum_to_file(&checksum_path, &checksum)?;
            info!("Calculated and saved checksum for existing JAR file: {jar_name}");
        }
    }
    
    info!("Validated existing JAR file: {jar_name}");
    Ok(())
}

fn download_and_validate_jar(download_url: &str, jar_path: &Path, jar_name: &str) -> Result<()> {
    download_file(download_url, jar_path, jar_name)?;
    
    let checksum_bytes = crate::utils::validation::validate_jar_and_calculate_checksum(jar_path)
        .with_context(|| format!("Failed to validate and calculate checksum for {jar_name}"))?;
    let checksum = hex::encode(checksum_bytes);
    
    let checksum_path = jar_path.with_extension("jar.sha256");
    save_checksum_to_file(&checksum_path, &checksum)?;
    
    info!("Downloaded JAR file checksum (SHA-256): {checksum}");
    info!("Validated downloaded JAR file: {jar_name}");
    
    Ok(())
}

pub fn download_jar(version: &str) -> Result<String> {
    let version_str = resolve_version(version)?;
    let latest_build = fetch_latest_build(&version_str)?;
    let download = fetch_download_info(&version_str, latest_build)?;

    let jar_name = &download.downloads.application.name;
    let jar_path = Path::new(jar_name);
    
    if jar_path.exists() {
        handle_existing_jar(jar_path, jar_name)?;
        return Ok(jar_name.to_string());
    }

    let download_url = format!(
        "{}/versions/{}/builds/{}/downloads/{}",
        API_BASE, version_str, latest_build, jar_name
    );

    download_and_validate_jar(&download_url, jar_path, jar_name)?;
    Ok(jar_name.to_string())
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
 