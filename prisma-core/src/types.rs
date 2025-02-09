pub mod channel;
pub mod extension;
pub mod item;
pub mod options;
pub mod platform;
pub mod provider;
pub mod version;

// .Store
pub const STORE_DIR: &str = "./.prisma";
pub const STORE_EXTENSIONS_DIR: &str = "./.prisma/extensions";
pub const STORE_TEMP_DIR: &str = "./.prisma/.temp";

pub const STORE_CORES_DIR: &str = "./.prisma/cors";
pub const STORE_PLUGINS_DIR: &str = "./.prisma/extensions/plugins";
pub const STORE_MODS_DIR: &str = "./.prisma/extensions/mods";

pub const STORE_PATH: &str = "./.prisma/store.ron";

// Config
pub const CONFIG_PATH: &str = "./prisma.ron";
