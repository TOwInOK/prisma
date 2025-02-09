use prisma_core::{
    extension::ExtensionType, item::Item, options::Options, platform::Platform, provider::Name,
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

impl Config {
    /// Push core version into extensions if is't latest.
    /// If extension has version, skip it.
    pub fn update_version(mut self) -> Self {
        if let Some(game_version) = self.core.version.game_version.as_ref() {
            self.extensions
                .iter_mut()
                .filter(|ext| ext.version.game_version.is_none())
                .for_each(|extension| extension.version.game_version = Some(game_version.clone()));
        }
        self
    }
    /// Change platrgorm for all extensions if needed
    pub fn update_platform(mut self) -> Self {
        for extension in self.extensions.iter_mut() {
            if extension.platform.is_none() {
                extension.platform = Some(self.core.platform.clone())
            }
        }
        self
    }
    /// Normalizes version & platform information across extensions
    ///
    pub fn normolise(self) -> Self {
        self.update_version().update_platform()
    }
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
    /// Platform for extension
    ///
    /// by default, providet by core
    pub platform: Option<Platform>,
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
            provider: prisma_core::provider::Provider::Extension((
                value.name,
                value.platform.unwrap_or_default(),
                value.provider,
            )),
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
