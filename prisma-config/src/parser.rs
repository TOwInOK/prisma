use crate::config::Config;

pub const DEFAULT_CONFIG_PATH: &str = "prisma.toml";

/// Parses a TOML configuration file into a Config struct
///
/// # Arguments
///
/// * `path` - Optional path to the config file. If not provided, uses [DEFAULT_CONFIG_PATH]
///
/// # Returns
///
/// * Result<[Config], Box<[dyn std::error::Error]>> - The parsed config or an error
pub async fn parse_config(
    path: Option<impl ToString>,
) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str =
        tokio::fs::read_to_string(path.map_or(DEFAULT_CONFIG_PATH.to_string(), |p| p.to_string()))
            .await?;
    toml::from_str(&config_str).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
