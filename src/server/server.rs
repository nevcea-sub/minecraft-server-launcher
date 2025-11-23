use std::process::{Command, Stdio};
use anyhow::Result;
use log::{error, warn};

use crate::constants::{JAVA_CMD, JAVA_JAR_ARG, JAVA_XMS_PREFIX, JAVA_XMX_PREFIX, RAM_UNIT};

const JAVA_ARGS: &[&str] = &[
    "-XX:+UseG1GC",
    "-XX:+ParallelRefProcEnabled",
    "-XX:MaxGCPauseMillis=200",
    "-XX:+UnlockExperimentalVMOptions",
    "-XX:+DisableExplicitGC",
    "-XX:+AlwaysPreTouch",
    "-XX:G1NewSizePercent=30",
    "-XX:G1MaxNewSizePercent=40",
    "-XX:G1HeapRegionSize=8M",
    "-XX:G1ReservePercent=20",
    "-XX:G1HeapWastePercent=5",
    "-XX:G1MixedGCCountTarget=4",
    "-XX:InitiatingHeapOccupancyPercent=15",
    "-XX:G1MixedGCLiveThresholdPercent=90",
    "-XX:G1RSetUpdatingPauseTimePercent=5",
    "-XX:SurvivorRatio=32",
    "-XX:+PerfDisableSharedMem",
    "-XX:MaxTenuringThreshold=1",
    "-Dusing.aikars.flags=https://mcflags.emc.gs",
    "-Daikars.new.flags=true",
    "-Dfile.encoding=UTF-8",
];

pub fn run_server(jar_file: &str, min_ram: u32, max_ram: u32, server_args: &[String]) -> Result<()> {
    let mut cmd = Command::new(JAVA_CMD);
    
    let min_ram_arg = format!("{}{}{}", JAVA_XMS_PREFIX, min_ram, RAM_UNIT);
    let max_ram_arg = format!("{}{}{}", JAVA_XMX_PREFIX, max_ram, RAM_UNIT);
    
    cmd.arg(&min_ram_arg)
        .arg(&max_ram_arg)
        .args(JAVA_ARGS)
        .arg(JAVA_JAR_ARG)
        .arg(jar_file)
        .args(server_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());

    match cmd.status() {
        Ok(status) => {
            if let Some(code) = status.code() {
                if code != 0 {
                    warn!("Server stopped with exit code: {}", code);
                }
            }
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            anyhow::bail!("Failed to start server: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_args_count() {
        assert!(JAVA_ARGS.len() > 0);
    }

    #[test]
    fn test_java_args_contains_g1gc() {
        assert!(JAVA_ARGS.contains(&"-XX:+UseG1GC"));
    }

    #[test]
    fn test_java_args_contains_encoding() {
        assert!(JAVA_ARGS.contains(&"-Dfile.encoding=UTF-8"));
    }

}

