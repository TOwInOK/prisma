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
pub enum Platform {
    // Mojang
    #[default]
    Vanilla,

    // Plugin group

    // base cores
    Spigot,
    Bukkit,

    // PaperMC group
    Paper,
    Folia,
    Waterfall,
    Velocity,

    // PaperMC like group
    Purpur,

    // Mod group
    Fabric,
    Quilt,
    Forge,
    NeoForge,
}
