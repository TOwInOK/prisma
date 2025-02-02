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
pub enum ExtensionProvider {
    // Plugins & mods
    #[default]
    Modrinth,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::AsRefStr,
    strum::IntoStaticStr,
    strum::EnumString,
    strum::EnumIter,
    strum::EnumIs,
)]
pub enum ExtensionType {
    Mod(ExtensionProvider),
    Plugin(ExtensionProvider),
}
