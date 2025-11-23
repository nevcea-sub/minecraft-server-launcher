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

## Performance

| Operation | Time | Improvement |
|-----------|------|-------------|
| JAR validation + checksum (integrated) | ~190µs | **22% faster** |
| JAR validation only | ~185µs | **12.8% faster** |
| Checksum validation (cached) | ~1µs | Cached in `.jar.sha256` |
| Checksum validation (calculated) | ~60µs | **20% faster** |

**Key Optimizations:**
- Integrated validation reads file only once (22% faster)
- Checksum caching reduces validation to ~1µs for subsequent runs
- Size-based buffer selection optimizes small and large files