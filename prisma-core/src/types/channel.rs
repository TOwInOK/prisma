#[derive(
    Debug,
    Clone,
    Copy,
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
/// Release channel
pub enum Channel {
    #[default]
    Release,
    Beta,
    Stable,
}
