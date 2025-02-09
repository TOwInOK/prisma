use prisma_core::{channel::Channel, item::Item, platform::Platform};
use prisma_hash::HashType;
use serde::{Deserialize, Serialize};

use crate::DownloadMeta;

/// # Example
/// we have cdn like this: `https://cdn.modrinth.com/data/PROJECT_ID/versions/ID/NAME-platform-VERSION.jar`
/// we can take `[project_id]` -> `AANobbMI`
/// we can take `[id]` -> `4GyXKCLd`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModrinthData {
    game_versions: Vec<String>,
    //Always change ich version
    id: String,
    //Stable token.
    // project_id: String,
    files: Vec<File>,
    // dependencies: Vec<Dependency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct File {
    hashes: Hashes,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Hashes {
    sha1: String,
    sha512: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Dependency {
    project_id: String,
    dependency_type: String,
}

impl ModrinthData {
    pub async fn get_link(
        name: &String,
        platform: &Platform,
        item: &Item,
    ) -> Result<DownloadMeta, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let channel = match &item.version.version_build {
            Some(channel) => channel.to_string(),
            None => Channel::Release.to_string(),
        };

        let link = format!("https://api.modrinth.com/v2/project/{}/version", name);

        let query = {
            match &item.version.game_version {
                Some(game_version) => vec![
                    ("game_version", format!("[\"{}\"]", game_version)),
                    ("loaders", format!("[\"{}\"]", platform)),
                    ("featured", true.to_string()),
                    ("version_type", channel),
                ],
                None => vec![
                    ("loaders", format!("[\"{}\"]", platform)),
                    ("featured", true.to_string()),
                    ("version_type", channel),
                ],
            }
        };
        let user_agent = format!("TOwInOK/Prisma UID: {}", machine_uid::get().unwrap());
        let client = reqwest::Client::builder().user_agent(user_agent).build()?;

        let modrinth_data: Vec<ModrinthData> =
            client.get(&link).query(&query).send().await?.json().await?;

        let modrinth_data = match modrinth_data.first() {
            Some(e) => e,
            None => Err(format!("Extension {} not found", name))?,
        };

        let version = modrinth_data
            .game_versions
            .first()
            .ok_or_else(|| format!("Not found any version of {}", modrinth_data.id))?
            .to_string();

        modrinth_data
            .files
            .first()
            .map(|x| DownloadMeta {
                download_link: x.url.to_string(),
                hash: HashType::new_sha1(x.hashes.sha1.to_string()),
                game_version: version,
                version_build: Some(modrinth_data.id.to_owned()),
            })
            .ok_or_else(|| format!("Download link for extension {} not found", name).into())
    }
}
