use super::{extension::ExtensionType, platform::Platform};

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
    strum::EnumIs,
)]
pub enum Provider {
    Core(Platform),
    Extension((Name, Platform, ExtensionType)),
}

pub type Name = String;
