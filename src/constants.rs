pub const CONFIG_FILE: &str = "config.toml";
pub const EULA_FILE: &str = "eula.txt";
pub const JAR_PREFIX: &str = "paper-";
pub const JAR_EXTENSION: &str = ".jar";
pub const DEFAULT_VERSION: &str = "latest";
pub const DEFAULT_SERVER_ARG: &str = "nogui";
pub const JAVA_CMD: &str = "java";
pub const JAVA_VERSION_ARG: &str = "-version";
pub const JAVA_JAR_ARG: &str = "-jar";
pub const JAVA_XMS_PREFIX: &str = "-Xms";
pub const JAVA_XMX_PREFIX: &str = "-Xmx";
pub const RAM_UNIT: &str = "G";
pub const CURRENT_DIR: &str = ".";

pub const DOWNLOAD_BUFFER_SIZE: usize = 64 * 1024;
pub const INPUT_BUFFER_CAPACITY: usize = 4;
pub const ERROR_PREVIEW_LENGTH: usize = 500;
pub const PROGRESS_UPDATE_INTERVAL: u64 = 1024;
pub const BYTES_PER_GB: u64 = 1024 * 1024 * 1024;

pub const RAM_LOW_THRESHOLD: u32 = 4;
pub const RAM_MID_THRESHOLD: u32 = 8;
pub const RAM_HIGH_THRESHOLD: u32 = 16;
pub const RAM_DEFAULT_MAX: u32 = 8;
pub const MIN_JAVA_VERSION: u32 = 17;
pub const DEFAULT_MIN_RAM: u32 = 2;
pub const DEFAULT_MAX_RAM: u32 = 4;
pub const MAX_RAM_LIMIT: u32 = 32;

pub const EULA_CONTENT: &str = "#By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).\neula=true\n";

pub const ENV_MINECRAFT_VERSION: &str = "MINECRAFT_VERSION";
pub const ENV_MIN_RAM: &str = "MIN_RAM";
pub const ENV_MAX_RAM: &str = "MAX_RAM";
pub const ENV_WORK_DIR: &str = "WORK_DIR";

pub const HTTP_TIMEOUT_SECS: u64 = 30;
pub const MIN_JAR_FILE_SIZE: u64 = 22;
pub const ZIP_MAGIC_PK1: u8 = 0x50;
pub const ZIP_MAGIC_PK2: u8 = 0x4B;
pub const ZIP_MAGIC_ARRAY_SIZE: usize = 4;
pub const HTTP_ERROR_PREVIEW_LENGTH: usize = 200;
pub const OPTIMAL_CHECKSUM_BUFFER_SIZE: usize = 64 * 1024;