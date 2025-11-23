# Minecraft Server Launcher

A fast and reliable Minecraft Paper server launcher written in Rust. This launcher automatically downloads Paper server JAR files, manages server configuration, and handles EULA acceptance.

## Features

- 🚀 **Automatic JAR Download**: Automatically downloads the latest Paper server JAR for your specified Minecraft version
- 💾 **Smart RAM Management**: Automatically calculates optimal RAM allocation based on system resources
- ☕ **Java Version Detection**: Verifies Java installation and version compatibility
- 📝 **EULA Handling**: Automatically accepts the Minecraft EULA
- 🔒 **File Integrity Verification**: SHA-256 checksum validation with caching for downloaded JAR files
- 🔐 **HTTPS Enforcement**: All downloads are performed over secure HTTPS connections

## Requirements

- **Java 17 or higher** (required to run Minecraft servers)
- **Windows, Linux, or macOS**

## Installation

### Download from GitHub Releases (Recommended)

1. Go to the [Releases](https://github.com/nevcea-sub/minecraft-server-launcher/releases) page
2. Download the latest `paper-launcher.exe` file
3. Run the executable

### Building from Source

1. Install [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)

2. Clone the repository:
```bash
git clone https://github.com/nevcea-sub/minecraft-server-launcher.git
cd minecraft-server-launcher
```

3. Build the project:
```bash
cargo build --release
```

4. The executable will be located at `target/release/paper-launcher.exe` (Windows) or `target/release/paper-launcher` (Linux/macOS)

## Usage

Run the launcher:
```bash
./paper-launcher
```

On first run, a `config.toml` file will be created with default settings. If no Paper JAR file is found, the launcher will automatically download it.

### Configuration

Edit `config.toml` to customize your server settings:

```toml
minecraft_version = "latest"  # Use "latest" for the latest version
min_ram = 2                   # Minimum RAM in GB
max_ram = 4                   # Maximum RAM in GB (auto-adjusted based on system RAM)
server_args = ["nogui"]       # Server arguments
# work_dir = "./server"       # Optional: working directory
```

### Command-Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--log-level` | `-l` | Log level: `trace`, `debug`, `info`, `warn`, `error` (default: `info`) |
| `--verbose` | | Enable verbose logging |
| `--quiet` | `-q` | Suppress all output except errors |
| `--config` | `-c` | Custom config file path |
| `--work-dir` | `-w` | Override working directory |
| `--version` | `-v` | Override Minecraft version |
| `--no-pause` | | Don't pause on exit |

### Environment Variables

Override configuration via environment variables: `MINECRAFT_VERSION`, `MIN_RAM`, `MAX_RAM`, `WORK_DIR`

```bash
export MINECRAFT_VERSION="1.21.1"
export MIN_RAM=4
export MAX_RAM=8
./paper-launcher
```

## Security

- 🔐 **HTTPS Enforcement**: All downloads over HTTPS only
- 🔒 **SHA-256 Checksum Validation**: JAR files verified against SHA-256 checksums (cached in `.jar.sha256`)
- ✅ **JAR Integrity Verification**: Validates ZIP structure, magic numbers, and manifest

## Performance Benchmarks

### Checksum Validation Performance

| Operation | Time | Improvement | Notes |
|-----------|------|-------------|-------|
| Checksum validation only | ~60µs | **20% faster** | Size-based buffer optimization |
| Checksum calculation (1KB) | ~58µs | Stable | Optimized for small files |
| Checksum calculation (1MB) | ~4.68ms | **3.4% faster** | Large buffer optimization |
| Checksum calculation (10MB) | ~49ms | Stable | Efficient for large files |
| Checksum validation (valid) | ~4.75ms | **4.5% faster** | Byte-level comparison |
| Checksum validation (invalid) | ~4.80ms | **5.6% faster** | Early detection optimization |

### JAR Validation Performance

| Operation | Time | Improvement | Notes |
|-----------|------|-------------|-------|
| JAR validation only | ~185µs | **12.8% faster** | Optimized ZIP parsing |
| JAR validation + checksum (integrated) | ~190µs | **22% faster** | Single file read |
| Checksum validation only | ~60µs | **8.8% faster** | Optimized buffer selection |

**Performance Comparison:**
- **Previous**: JAR validation (185µs) + checksum validation (58µs) = **243µs**
- **Current**: Integrated function = **190µs**
- **Result**: **22% faster** by reading file only once

> **Note**: Checksums are cached in `.jar.sha256` files. Subsequent validations only require reading the cached checksum file (~1µs) instead of recalculating the hash.

### Summary

- ✅ Sub-millisecond latency for typical JAR files
- ✅ Checksum validation: **~60µs** overhead
- ✅ Integrated validation: **22% faster** than separate operations