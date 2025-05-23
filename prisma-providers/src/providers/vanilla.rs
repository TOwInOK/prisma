use prisma_core::item::Item;
use prisma_hash::HashType;
use serde::{Deserialize, Serialize};

use crate::DownloadMeta;

/// Structure representing Minecraft version manifest
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vanilla {
    latest: Latest,
    versions: Vec<Version>,
}

/// Latest available Minecraft versions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Latest {
    release: String,
    snapshot: String,
}

/// Individual Minecraft version details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Version {
    #[serde(rename = "id")]
    version: String,

    #[serde(rename = "type")]
    type_field: TypeOfVersion,

    url: String,
}

/// Minecraft version types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
enum TypeOfVersion {
    #[serde(rename = "release")]
    #[default]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha,
}

/// Download details for a specific version
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DownloadSection {
    downloads: Downloads,
}

/// Available download types
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Downloads {
    server: Server,
}

/// Server download details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Server {
    sha1: String,
    url: String,
}
impl Vanilla {
    /// Retrieves download link and metadata for a Minecraft server jar
    ///
    /// Makes request to Mojang API to find the download link for minecraft.jar
    /// Returns DownloadMeta containing URL, hash and version info
    pub async fn get_link(
        item: &Item,
    ) -> Result<DownloadMeta, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let link = find_version(item.version.game_version.as_deref()).await?;
        let response = reqwest::get(link.0).await?;
        let download_section: DownloadSection = response.json().await?;

        Ok(DownloadMeta {
            download_link: download_section.downloads.server.url,
            hash: HashType::new_sha1(download_section.downloads.server.sha1),
            game_version: link.1,
            version_build: None,
        })
    }
}

/// Finds version-specific download information
///
/// Makes request to version manifest and returns tuple of (download URL, version string)
/// If no version specified, returns latest release version
async fn find_version(
    version: Option<&str>,
) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    const LINK: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

    let response = reqwest::get(LINK).await?;
    let vanilla: Vanilla = response.json().await?;
    let local_version = match version {
        Some(e) => e.to_owned(),
        None => vanilla.latest.release,
    };

    vanilla
        .versions
        .iter()
        .find(|x| x.version.contains(&local_version))
        .map(|x| (x.version.clone(), x.url.clone()))
        .ok_or_else(|| format!("Version {} not found", local_version).into())
}
