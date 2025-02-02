use super::{
    extension::{ExtensionProvider, ExtensionType, Platform},
    options::Options,
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
//
// ```
// let vanilla = Item::new_core(CoreProvider::Vanilla)
//     .with_version(Version::Latest);
//
// let mod_item = Item::new_mod(ExtensionProvider::Modrinth)
//     .with_version(Version::Specific("1.0.0".to_string(), None))
//     .with_options(Options { freeze: true, force_update: false });
//
// let plugin = Item::new_plugin(ExtensionProvider::default())
//     .with_version(Version::default());
// ```
impl Item {
    pub fn new_core(provider: Platform) -> Self {
        Self {
            provider: Provider::Core(provider),
            version: Version::default(),
            options: Options::default(),
        }
    }

    pub fn new_mod(name: Name, provider: ExtensionProvider) -> Self {
        Self {
            provider: Provider::Extension((name, ExtensionType::Mod(provider))),
            version: Version::default(),
            options: Options::default(),
        }
    }

    pub fn new_plugin(name: Name, provider: ExtensionProvider) -> Self {
        Self {
            provider: Provider::Extension((name, ExtensionType::Plugin(provider))),
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

// IS helpers
impl Item {
    pub fn is_core(&self) -> bool {
        matches!(self.provider, Provider::Core(_))
    }

    pub fn is_mod(&self) -> bool {
        matches!(
            self.provider,
            Provider::Extension((_, ExtensionType::Mod(_)))
        )
    }

    pub fn is_plugin(&self) -> bool {
        matches!(
            self.provider,
            Provider::Extension((_, ExtensionType::Plugin(_)))
        )
    }
}
