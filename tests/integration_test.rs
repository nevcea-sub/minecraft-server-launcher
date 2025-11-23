use minecraft_server_launcher::config::Config;
use minecraft_server_launcher::utils::validation::validate_jar_file;
use minecraft_server_launcher::utils::checksum::{calculate_file_sha256, validate_file_checksum};
use minecraft_server_launcher::download::http::validate_https_url;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_https_validation() {
    assert!(validate_https_url("https://example.com/file.jar").is_ok());
    assert!(validate_https_url("http://example.com/file.jar").is_err());
}

#[test]
fn test_config_load_with_invalid_work_dir() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
minecraft_version = "1.21.1"
min_ram = 2
max_ram = 4
server_args = ["nogui"]
work_dir = "/nonexistent/path/that/does/not/exist"
"#;
    fs::write(&config_path, config_content).unwrap();
    
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    let config = Config::load_from_path(&config_path).unwrap();
    assert_eq!(config.work_dir, Some("/nonexistent/path/that/does/not/exist".to_string()));
    
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_jar_validation_edge_cases() {
    let temp_dir = TempDir::new().unwrap();
    
    let small_jar = temp_dir.path().join("small.jar");
    fs::write(&small_jar, b"PK").unwrap();
    assert!(validate_jar_file(&small_jar).is_err());
    
    let invalid_jar = temp_dir.path().join("invalid.jar");
    let mut content = vec![0x50, 0x4B];
    content.extend(vec![0u8; 20]);
    fs::write(&invalid_jar, content).unwrap();
    assert!(validate_jar_file(&invalid_jar).is_err());
}

#[test]
fn test_checksum_calculation_edge_cases() {
    let temp_dir = TempDir::new().unwrap();
    
    let empty_file = temp_dir.path().join("empty.txt");
    fs::write(&empty_file, b"").unwrap();
    let hash = calculate_file_sha256(&empty_file).unwrap();
    assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(hash.len(), 64);
    
    let large_file = temp_dir.path().join("large.txt");
    let content = vec![0u8; 10000];
    fs::write(&large_file, content).unwrap();
    let hash = calculate_file_sha256(&large_file).unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_checksum_validation() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "test content").unwrap();
    
    let valid_hash = calculate_file_sha256(&file).unwrap();
    assert!(validate_file_checksum(&file, Some(&valid_hash)).is_ok());
    
    assert!(validate_file_checksum(&file, Some("invalid_hash")).is_err());
    
    assert!(validate_file_checksum(&file, None).is_ok());
}

#[test]
fn test_config_validation_edge_cases() {
    let config = Config {
        min_ram: 4,
        max_ram: 4,
        ..Config::default()
    };
    assert!(config.validate().is_ok());
    
    let config = Config {
        max_ram: 32,
        ..Config::default()
    };
    assert!(config.validate().is_ok());
    
    let config = Config {
        max_ram: 33,
        ..Config::default()
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_config_env_override_edge_cases() {
    std::env::set_var("MIN_RAM", "1000");
    std::env::set_var("MAX_RAM", "2000");
    
    let mut config = Config::default();
    Config::apply_env_overrides(&mut config);
    
    assert_eq!(config.min_ram, 1000);
    assert_eq!(config.max_ram, 2000);
    
    std::env::remove_var("MIN_RAM");
    std::env::remove_var("MAX_RAM");
}

#[test]
fn test_jar_file_finding_with_multiple_jars() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    let jar1 = temp_dir.path().join("paper-1.21.1-100.jar");
    let jar2 = temp_dir.path().join("paper-1.21.1-101.jar");
    
    use std::io::Write;
    let mut file1 = std::fs::File::create(&jar1).unwrap();
    let mut file2 = std::fs::File::create(&jar2).unwrap();
    
    file1.write_all(&[0x50, 0x4B, 0x03, 0x04]).unwrap();
    file1.write_all(&vec![0u8; 18]).unwrap();
    
    file2.write_all(&[0x50, 0x4B, 0x03, 0x04]).unwrap();
    file2.write_all(&vec![0u8; 18]).unwrap();
    
    drop(file1);
    drop(file2);
    
    let result = minecraft_server_launcher::utils::find_jar_file();
    assert!(result.is_ok());

    let found = result.unwrap();
    assert!(found.is_some());
    
    std::env::set_current_dir(original_dir).unwrap();
}
