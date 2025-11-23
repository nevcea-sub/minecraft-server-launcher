use sysinfo::System;
use anyhow::Result;

use crate::constants::{BYTES_PER_GB, RAM_DEFAULT_MAX, RAM_HIGH_THRESHOLD, RAM_LOW_THRESHOLD, RAM_MID_THRESHOLD};

pub fn get_total_ram_gb() -> Result<Option<u32>> {
    let mut system = System::new();
    system.refresh_memory();
    
    let total_gb = (system.total_memory() / BYTES_PER_GB) as u32;
    Ok((total_gb > 0).then_some(total_gb))
}

pub fn calculate_max_ram(original_max: u32, total_ram_gb: Option<u32>, min_ram: u32) -> u32 {
    let max_ram = match total_ram_gb {
        Some(total) if total <= RAM_LOW_THRESHOLD => (total / 2).max(min_ram),
        Some(total) if total <= RAM_MID_THRESHOLD => total / 2,
        Some(total) if total <= RAM_HIGH_THRESHOLD => RAM_DEFAULT_MAX,
        Some(total) => (total / 2).min(RAM_HIGH_THRESHOLD),
        None => original_max / 2,
    };

    max_ram.max(min_ram)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_max_ram_basic() {
        assert_eq!(calculate_max_ram(4, Some(8), 2), 4);
        assert_eq!(calculate_max_ram(4, None, 2), 2);
    }
}

