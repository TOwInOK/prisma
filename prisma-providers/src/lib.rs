use prisma_core::{
    extension::{ExtensionProvider, ExtensionType},
    item::Item,
    platform::Platform,
    provider::Provider,
};
use prisma_hash::HashType;
use providers::{modrinth::ModrinthData, papermc::PaperMC, purpur::Purpur, vanilla::Vanilla};

pub mod providers;
pub struct DownloadMeta {
    /// link to download jar file
    pub download_link: String,
    /// hash of this file
    pub hash: HashType,
    /// latest version
    ///
    /// like latest minecraft version
    pub game_version: String,
    /// build if it exist
    ///
    /// like lastest build if minecraft version
    pub version_build: Option<String>,
}

impl DownloadMeta {
    pub async fn fetch(value: &Item) -> Result<Self, Box<dyn std::error::Error>> {
        match &value.provider {
            Provider::Core(platform) => match platform {
                Platform::Vanilla => Vanilla::get_link(value).await,
                Platform::Spigot => unimplemented!("Need to implement for Spigot platform"),
                Platform::Bukkit => unimplemented!("Need to implement for Bukkit platform"),
                Platform::Paper => PaperMC::get_link(value).await,
                Platform::Folia => PaperMC::get_link(value).await,
                Platform::Waterfall => PaperMC::get_link(value).await,
                Platform::Velocity => PaperMC::get_link(value).await,
                Platform::Purpur => Purpur::get_link(value).await,
                Platform::Fabric => unimplemented!("Need to implement for Fabric platform"),
                Platform::Quilt => unimplemented!("Need to implement for Quilt platform"),
                Platform::Forge => unimplemented!("Need to implement for Forge platform"),
                Platform::NeoForge => unimplemented!("Need to implement for NeoForge platform"),
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
