# Minecraft Server Launcher

A fast and reliable Minecraft Paper server launcher written in Rust. This launcher automatically downloads Paper server JAR files, manages server configuration, and handles EULA acceptance.

## Features

- 🚀 **Automatic JAR Download**: Automatically downloads the latest Paper server JAR for your specified Minecraft version
- ⚙️ **Configuration Management**: Simple TOML-based configuration file
- 💾 **Smart RAM Management**: Automatically calculates optimal RAM allocation based on system resources
- ☕ **Java Version Detection**: Verifies Java installation and version compatibility
- 📝 **EULA Handling**: Automatically accepts the Minecraft EULA
- 🔧 **Environment Variable Overrides**: Override configuration via environment variables
- 📊 **Progress Indicators**: Visual download progress with progress bars

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

### First Run

1. Run the launcher:
```bash
./paper-launcher
```

2. On first run, a `config.toml` file will be created with default settings. Edit it to customize your server configuration.

3. If no Paper JAR file is found, the launcher will prompt you to download it automatically.

### Configuration

Edit `config.toml` to customize your server settings:

```toml
# Minecraft version (use "latest" for the latest version)
minecraft_version = "latest"

# Minimum RAM in GB
min_ram = 2

# Maximum RAM in GB (will be auto-adjusted based on system RAM)
max_ram = 4

# Server arguments
server_args = ["nogui"]

# Working directory (optional, defaults to current directory)
# work_dir = "./server"
```

### Environment Variables

You can override configuration values using environment variables:

- `MINECRAFT_VERSION`: Override the Minecraft version
- `MIN_RAM`: Override minimum RAM (in GB)
- `MAX_RAM`: Override maximum RAM (in GB)
- `WORK_DIR`: Override the working directory

Example:
```bash
export MINECRAFT_VERSION="1.21.1"
export MIN_RAM=4
export MAX_RAM=8
./paper-launcher
```

## How It Works

1. **Configuration Loading**: Loads settings from `config.toml` or creates a default one
2. **Java Check**: Verifies Java installation and version (requires Java 17+)
3. **JAR Detection**: Checks for existing Paper JAR files in the working directory
4. **Auto-Download**: If no JAR is found, downloads the latest Paper build for the specified version
5. **EULA Handling**: Automatically accepts the Minecraft EULA
6. **RAM Calculation**: Calculates optimal RAM allocation based on system resources
7. **Server Launch**: Starts the Minecraft server with the configured settings

## Project Structure

```
minecraft-server-launcher/
├── src/
│   ├── main.rs           # Main entry point
│   ├── lib.rs            # Library root
│   ├── api/              # Paper API integration
│   ├── config/           # Configuration management
│   ├── download/         # JAR download functionality
│   ├── server/           # Server management and Java handling
│   └── utils/            # Utility functions
├── tests/                # Integration tests
├── Cargo.toml            # Rust dependencies
└── LICENSE.md            # GPL v3 License
```

## Dependencies

- `reqwest` - HTTP client for downloading JAR files
- `serde` / `serde_json` - JSON serialization/deserialization
- `toml` - TOML configuration parsing
- `sysinfo` - System information (RAM detection)
- `indicatif` - Progress bars
- `clap` - Command-line argument parsing
- `anyhow` - Error handling
- `log` / `env_logger` - Logging