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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_string() {
        let inst = Provider::Core(Platform::default());
        println!("display - {}", inst); // Out -> Core
        println!("string - {}", inst.as_ref()); // Out -> Core
        println!(
            "platform - {:#?}",
            match inst {
                Provider::Core(platform) => platform.to_string(),
                Provider::Extension(e) => format!("{} - {} - {}", e.0, e.1, e.2),
            }
        ) // Out -> Vanilla
    }
}
