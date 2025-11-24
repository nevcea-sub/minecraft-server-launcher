# Minecraft Server Launcher

A fast and reliable Minecraft Paper server launcher written in Go.

## Features

- Automatic JAR download and update management
- Smart RAM allocation based on system resources
- Java version validation (Java 17+)
- SHA-256 checksum verification
- Automatic world backups
- EULA auto-acceptance

## Requirements

- Java 17 or higher
- Windows, Linux, or macOS

## Installation

### Download from GitHub Releases

1. Go to the [Releases](https://github.com/nevcea-sub/minecraft-server-launcher/releases) page
2. Download the appropriate binary for your OS
3. Run the executable

### Building from Source

```bash
git clone https://github.com/nevcea-sub/minecraft-server-launcher.git
cd minecraft-server-launcher
go build -o paper-launcher .
```

## Usage

```bash
./paper-launcher
```

On first run, a `config.yaml` file will be created with default settings.

### Configuration

Edit `config.yaml`:

```yaml
minecraft_version: "latest"
auto_update: false
auto_backup: true
backup_count: 10
backup_worlds:
  - world
  - world_nether
  - world_the_end
min_ram: 2
max_ram: 0
use_zgc: false
auto_ram_percentage: 85
server_args:
  - nogui
```

### Command-Line Options

```
  -c string    Custom config file path (default "config.yaml")
  -w string    Override working directory
  -v string    Override Minecraft version
  -q           Suppress all output except errors
  -verbose     Enable verbose logging
  -no-pause    Don't pause on exit
```

### Environment Variables

- `MINECRAFT_VERSION`: Override Minecraft version
- `WORK_DIR`: Override working directory
- `MIN_RAM`: Override minimum RAM
- `MAX_RAM`: Override maximum RAM

## License

GPL-3.0 License - See [LICENSE.md](LICENSE.md) for details
