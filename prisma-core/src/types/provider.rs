use super::extension::{ExtensionType, Platform};

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
    Extension((Name, ExtensionType)),
}

pub type Name = String;
