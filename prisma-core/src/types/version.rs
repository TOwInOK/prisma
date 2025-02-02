use super::channel::Channel;

#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    Default,
    strum::Display,
    strum::AsRefStr,
    strum::IntoStaticStr,
    strum::EnumString,
    strum::EnumIter,
    strum::EnumIs,
)]
pub enum Version {
    #[default]
    /// Latest version, latest build, only release
    Latest,
    /// Version, Build number, Channel
    /// - if version is none -> Latest
    /// - if version build is none -> Latest
    /// - if channel is none -> Release
    ///
    /// - if version & version build isn't empty -> version build
    Specific(
        Option<GameVersion>,
        Option<GameVersionBuild>,
        Option<Channel>,
    ),
}

pub type GameVersion = String;
pub type GameVersionBuild = String;
