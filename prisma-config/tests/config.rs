#[cfg(test)]
mod config {
    use prisma_config::config::{
        Config, CoreConfig, CoreOptions, Difficulty, ExtensionConfig, Gamemode, ServerProperties,
    };
    use prisma_core::{extension::ExtensionType, platform::Platform, version::Version};
    use ron::ser::PrettyConfig;

    #[test]
    fn gen_default() {
        let config = Config::new()
            .with_core(CoreConfig {
                platform: Platform::Paper,
                version: Version {
                    game_version: Some("1.20.1".to_string()),
                    version_build: Some("17".to_string()),
                    ..Default::default()
                },
                options: Default::default(),
            })
            .with_options(CoreOptions {
                port: 25565,
                min_memory: 2048,
                max_memory: 4096,
                java_args: vec![
                    "-XX:+UseG1GC".to_string(),
                    "-XX:+ParallelRefProcEnabled".to_string(),
                    "-XX:MaxGCPauseMillis=200".to_string(),
                ],
                properties: ServerProperties {
                    motd: Some("A Minecraft Server powered by Prisma".to_string()),
                    max_players: Some(20),
                    online_mode: Some(true),
                    difficulty: Some(Difficulty::Normal),
                    gamemode: Some(Gamemode::Survival),
                    view_distance: Some(10),
                    allow_nether: Some(true),
                    enable_command_block: Some(false),
                },
            })
            .add_extension(ExtensionConfig {
                name: "worldedit".to_string(),
                platform: None, // Will be inherited from core
                provider: ExtensionType::Plugin(
                    prisma_core::extension::ExtensionProvider::Modrinth,
                ),
                version: Version {
                    game_version: None, // Will be inherited from core
                    ..Default::default()
                },
                options: Default::default(),
            })
            .add_extension(ExtensionConfig {
                name: "vault".to_string(),
                platform: None,
                provider: ExtensionType::Plugin(
                    prisma_core::extension::ExtensionProvider::default(),
                ),
                version: Version {
                    game_version: None,
                    ..Default::default()
                },
                options: Default::default(),
            })
            .normolise();

        let toml = toml::to_string_pretty(&config).unwrap();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let ron =
            ron::ser::to_string_pretty(&config, PrettyConfig::default().enumerate_arrays(true))
                .unwrap();
        std::fs::write("test_config.toml", toml).unwrap();
        std::fs::write("test_config.json", json).unwrap();
        std::fs::write("test_config.ron", ron).unwrap();
    }
}
