package config

import (
	"fmt"
	"os"

	"github.com/BurntSushi/toml"
)

const defaultConfig = `# Minecraft Server Launcher Configuration
# Edit this file to customize server settings

# Minecraft version (use "latest" for the latest version)
minecraft_version = "latest"

# Auto update Paper server (Check for updates on startup)
# true: Automatically download new builds
# false: Ask user before updating (default)
auto_update = false

# Minimum RAM in GB
min_ram = 2

# Maximum RAM in GB (set to 0 for auto-calculation based on available system RAM)
max_ram = 0

# Use ZGC garbage collector (Recommended for high RAM, requires Java 15+)
use_zgc = false

# Percentage of system RAM to use when max_ram is 0 (Auto mode)
# Default: 85 (Uses 85% of available RAM)
auto_ram_percentage = 85

# Server arguments
server_args = ["nogui"]

# Working directory (optional, defaults to current directory)
# work_dir = "./server"
`

type Config struct {
	MinecraftVersion  string   `toml:"minecraft_version"`
	AutoUpdate        bool     `toml:"auto_update"`
	MinRAM            int      `toml:"min_ram"`
	MaxRAM            int      `toml:"max_ram"`
	UseZGC            bool     `toml:"use_zgc"`
	AutoRAMPercentage int      `toml:"auto_ram_percentage"`
	ServerArgs        []string `toml:"server_args"`
	WorkDir           string   `toml:"work_dir"`
}

func Load(path string) (*Config, error) {
	if _, err := os.Stat(path); os.IsNotExist(err) {
		if err := os.WriteFile(path, []byte(defaultConfig), 0644); err != nil {
			return nil, fmt.Errorf("failed to create config: %w", err)
		}
		fmt.Println("Created config.toml with default settings.")
	}

	var cfg Config
	cfg.AutoRAMPercentage = 85
	cfg.AutoUpdate = false

	if _, err := toml.DecodeFile(path, &cfg); err != nil {
		return nil, fmt.Errorf("failed to parse config: %w", err)
	}

	if v := os.Getenv("MINECRAFT_VERSION"); v != "" {
		cfg.MinecraftVersion = v
	}
	if v := os.Getenv("WORK_DIR"); v != "" {
		cfg.WorkDir = v
	}
	
	if v := os.Getenv("MIN_RAM"); v != "" {
		var minRAM int
		if _, err := fmt.Sscanf(v, "%d", &minRAM); err == nil {
			cfg.MinRAM = minRAM
		}
	}
	if v := os.Getenv("MAX_RAM"); v != "" {
		var maxRAM int
		if _, err := fmt.Sscanf(v, "%d", &maxRAM); err == nil {
			cfg.MaxRAM = maxRAM
		}
	}

	if err := cfg.Validate(); err != nil {
		return nil, err
	}

	return &cfg, nil
}

func (c *Config) Validate() error {
	if c.MinecraftVersion == "" {
		return fmt.Errorf("minecraft_version cannot be empty")
	}
	if c.MinRAM <= 0 {
		return fmt.Errorf("min_ram must be greater than 0")
	}
	if c.MaxRAM != 0 && c.MinRAM > c.MaxRAM {
		return fmt.Errorf("min_ram cannot be greater than max_ram")
	}
	if c.MaxRAM > 128 {
		return fmt.Errorf("max_ram exceeds safety limit (128GB)")
	}
	if c.AutoRAMPercentage < 10 || c.AutoRAMPercentage > 95 {
		return fmt.Errorf("auto_ram_percentage must be between 10 and 95")
	}
	return nil
}
