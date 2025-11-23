use std::io::{self, Write};
use anyhow::{Context, Result};

use crate::constants::INPUT_BUFFER_CAPACITY;

pub fn pause() -> Result<()> {
    print!("Press Enter to continue...");
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut buffer = String::with_capacity(INPUT_BUFFER_CAPACITY);
    io::stdin().read_line(&mut buffer).context("Failed to read input")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_buffer_capacity() {
        let _buffer = String::with_capacity(INPUT_BUFFER_CAPACITY);
    }
}

