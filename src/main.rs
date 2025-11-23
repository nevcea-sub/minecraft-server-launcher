mod api;
mod config;
mod constants;
mod download;
mod server;
mod utils;

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use log::{error, info};

use crate::config::Config;
use crate::constants::{CONFIG_FILE, CURRENT_DIR, INPUT_BUFFER_CAPACITY};
use crate::server::{check_java, calculate_max_ram, get_total_ram_gb};
use crate::utils::{download_jar, find_jar_file, handle_eula};
use crate::server::run_server;
use crate::utils::pause;

fn get_exe_directory() -> Result<PathBuf> {
    std::env::current_exe()
        .context("Failed to get executable path")?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get executable directory"))
        .map(PathBuf::from)
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Warn)
        .init();

    if let Err(e) = run() {
        let error_str = e.to_string();
        if !error_str.contains("Cannot start server without JAR file") {
            error!("Fatal error: {e}");
            eprintln!("\n[ERROR] {e}");
        }
        let _ = pause();
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = setup_working_directory()?;
    
    check_prerequisites(&config)?;
    
    let (jar_file, config) = ensure_jar_file(config)?;
    handle_eula()?;
    
    start_server(&jar_file, config)?;
    pause()?;
    Ok(())
}

fn setup_working_directory() -> Result<Config> {
    let exe_dir = get_exe_directory()?;
    std::env::set_current_dir(&exe_dir)
        .with_context(|| format!("Failed to change to executable directory: {}", exe_dir.display()))?;

    let config = Config::load()?;
    let work_dir = config.work_directory();
    
    if work_dir != Path::new(CURRENT_DIR) {
        std::env::set_current_dir(&work_dir)
            .with_context(|| format!("Failed to change to work directory: {}", work_dir.display()))?;
        info!("Changed working directory to: {}", work_dir.display());
    }
    
    Ok(config)
}

fn check_prerequisites(_config: &Config) -> Result<()> {
    let java_version = check_java()?;
    info!("Java version: {java_version}");
    
    let total_ram_gb = get_total_ram_gb();
    if let Some(ram) = total_ram_gb {
        info!("Total system RAM: {ram} GB");
    }
    
    Ok(())
}

fn ensure_jar_file(mut config: Config) -> Result<(String, Config)> {
    if let Some(jar) = find_jar_file()? {
        info!("Found JAR file: {jar}");
        Ok((jar, config))
    } else {
            let was_auto_created = config.auto_created;
            config.reload()?;
            config.auto_created = was_auto_created;
            
            print!("No Paper JAR file found. Download automatically? [Y/N]: ");
            io::stdout().flush().context("Failed to flush stdout")?;

            let mut input = String::with_capacity(INPUT_BUFFER_CAPACITY);
            io::stdin().read_line(&mut input).context("Failed to read input")?;
            
            if input.trim().eq_ignore_ascii_case("y") {
                info!("Downloading JAR file for version: {}", config.minecraft_version);
                let jar = download_jar(&config.minecraft_version)?;
                Ok((jar, config))
            } else {
                cleanup_auto_created_config(&config)?;
                println!("\nJAR file is required to start the server.");
                println!("You can:");
                println!("  1. Run this launcher again and choose 'Y' to download automatically");
                println!("  2. Download Paper JAR manually from https://papermc.io/downloads");
                println!("  3. Place the JAR file in the current directory");
                anyhow::bail!("Cannot start server without JAR file.");
            }
        }
    }

fn cleanup_auto_created_config(config: &Config) -> Result<()> {
    if config.auto_created {
        let config_path = Path::new(CONFIG_FILE);
        if config_path.exists() {
            std::fs::remove_file(config_path)
                .context("Failed to remove auto-created config.toml")?;
            info!("Removed auto-created config.toml");
        }
    }
    Ok(())
}

fn start_server(jar_file: &str, mut config: Config) -> Result<()> {
    config.reload()?;
    
    let total_ram_gb = get_total_ram_gb();
    let max_ram = calculate_max_ram(config.max_ram, total_ram_gb, config.min_ram);
    
    info!("Starting server with {}G - {}G RAM", config.min_ram, max_ram);
    info!("JAR file: {jar_file}");
    
    run_server(jar_file, config.min_ram, max_ram, &config.server_args)?;
    Ok(())
}
