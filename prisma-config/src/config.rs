use prisma_core::{
    extension::{ExtensionType, Platform},
    item::Item,
    options::Options,
    provider::Name,
    version::Version,
};
use serde::{Deserialize, Serialize};

/// Configuration for a Minecraft server instance
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Core configuration
    pub core: CoreConfig,
    /// Extensions configuration (plugins and mods)
    pub extensions: Vec<ExtensionConfig>,
    /// Server core options
    pub options: CoreOptions,
}

/// Core configuration for the Minecraft server
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoreConfig {
    /// Platform type (Vanilla, Paper, Fabric, etc)
    pub platform: Platform,
    /// Version configuration
    pub version: Version,
    /// Core specific options
    pub options: Options,
}

/// Configuration for server extensions like mods and plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    /// Extension name
    pub name: Name,
    /// Extension provider (Modrinth, etc)
    pub provider: ExtensionType,
    /// Version configuration
    #[serde(default)]
    pub version: Version,
    /// Extension specific options
    #[serde(default)]
    pub options: Options,
}

/// Converts CoreConfig into a generic Item
impl From<CoreConfig> for Item {
    fn from(value: CoreConfig) -> Self {
        Self {
            provider: prisma_core::provider::Provider::Core(value.platform),
            version: value.version,
            options: value.options,
        }
    }
}

/// Converts ExtensionConfig into a generic Item
impl From<ExtensionConfig> for Item {
    fn from(value: ExtensionConfig) -> Self {
        Self {
            provider: prisma_core::provider::Provider::Extension((value.name, value.provider)),
            version: value.version,
            options: value.options,
        }
    }
}

/// Core options for configuring the Minecraft server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreOptions {
    /// Server port
    pub port: u16,
    /// Minimum RAM allocation (in MB)
    pub min_memory: u32,
    /// Maximum RAM allocation (in MB)
    pub max_memory: u32,
    /// Java arguments
    pub java_args: Vec<String>,
    /// Server properties
    pub properties: ServerProperties,
}

/// Server properties for configuring Minecraft server behavior
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerProperties {
    /// Server MOTD
    pub motd: Option<String>,
    /// Max players
    pub max_players: Option<u32>,
    /// Online mode
    pub online_mode: Option<bool>,
    /// Difficulty
    pub difficulty: Option<Difficulty>,
    /// Gamemode
    pub gamemode: Option<Gamemode>,
    /// View distance
    pub view_distance: Option<u32>,
    /// Allow nether
    pub allow_nether: Option<bool>,
    /// Enable command blocks
    pub enable_command_block: Option<bool>,
}

/// Server difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

/// Server game modes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Default for CoreOptions {
    fn default() -> Self {
        Self {
            port: 25565,
            min_memory: 1024,
            max_memory: 2048,
            java_args: vec!["-XX:+UseG1GC".to_string()],
            properties: ServerProperties::default(),
        }
    }
}

impl Config {
    /// Creates a new default Config instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the core configuration
    pub fn with_core(mut self, core: CoreConfig) -> Self {
        self.core = core;
        self
    }

    /// Adds an extension to the configuration
    pub fn add_extension(mut self, extension: ExtensionConfig) -> Self {
        self.extensions.push(extension);
        self
    }

    /// Sets the core options
    pub fn with_options(mut self, options: CoreOptions) -> Self {
        self.options = options;
        self
    }
}
