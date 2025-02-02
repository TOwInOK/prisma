use prisma_core::{
    extension::{ExtensionProvider, ExtensionType},
    item::Item,
    platform::Platform,
    provider::Provider,
};
use prisma_hash::HashType;
use providers::{modrinth::ModrinthData, vanilla::Vanilla};

pub mod providers;
pub struct DownloadMeta {
    /// link to download jar file
    pub download_link: String,
    /// hash of this file
    pub hash: HashType,
    /// latest version
    ///
    /// like latest minecraft version
    pub version: String,
    /// build if it exist
    ///
    /// like lastest build if minecraft version
    pub build: Option<String>,
}

impl DownloadMeta {
    pub async fn from(value: &Item) -> Result<Self, Box<dyn std::error::Error>> {
        match &value.provider {
            Provider::Core(platform) => match platform {
                Platform::Vanilla => Vanilla::get_link(value).await,
                Platform::Spigot => todo!(),
                Platform::Bukkit => todo!(),
                Platform::Paper => todo!(),
                Platform::Folia => todo!(),
                Platform::Waterfall => todo!(),
                Platform::Velocity => todo!(),
                Platform::Purpur => todo!(),
                Platform::Fabric => todo!(),
                Platform::Quilt => todo!(),
                Platform::Forge => todo!(),
                Platform::NeoForge => todo!(),
            },
            Provider::Extension((name, platform, extension_type)) => match extension_type {
                ExtensionType::Mod(extension_provider) => match extension_provider {
                    ExtensionProvider::Modrinth => {
                        ModrinthData::get_link(name, platform, value).await
                    }
                },
                ExtensionType::Plugin(extension_provider) => match extension_provider {
                    ExtensionProvider::Modrinth => {
                        ModrinthData::get_link(name, platform, value).await
                    }
                },
            },
        }
    }
}
