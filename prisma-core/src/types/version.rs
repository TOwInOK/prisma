use super::channel::Channel;

pub type GameVersion = String;
pub type GameVersionBuild = String;

/// Version, Build number, Channel
/// - if version is none -> Latest
/// - if version build is none -> Latest
/// - if channel is none -> Release
///
/// - if version & version build isn't empty -> version build
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Version {
    pub game_version: Option<String>,
    pub version_build: Option<String>,
    pub channel: Channel,
}
