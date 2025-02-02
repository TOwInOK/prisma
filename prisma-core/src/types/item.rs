use super::{
    extension::{ExtensionProvider, ExtensionType},
    options::Options,
    platform::Platform,
    provider::{Name, Provider},
    version::Version,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Item {
    /// Откуда мы качаем
    pub provider: Provider,
    /// Что за версию скачать и с каким билдом
    pub version: Version,
    /// Дополнительные параметры
    pub options: Options,
}

// builder helpers
impl Item {
    pub fn new_core(provider: Platform) -> Self {
        Self {
            provider: Provider::Core(provider),
            version: Version::default(),
            options: Options::default(),
        }
    }

    pub fn new_mod(name: Name, platform: Platform, provider: ExtensionProvider) -> Self {
        Self {
            provider: Provider::Extension((name, platform, ExtensionType::Mod(provider))),
            version: Version::default(),
            options: Options::default(),
        }
    }

    pub fn new_plugin(name: Name, platform: Platform, provider: ExtensionProvider) -> Self {
        Self {
            provider: Provider::Extension((name, platform, ExtensionType::Plugin(provider))),
            version: Version::default(),
            options: Options::default(),
        }
    }

    // Builder-style методы
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    pub fn with_options(mut self, options: Options) -> Self {
        self.options = options;
        self
    }
}
