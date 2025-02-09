use prisma_core::{item::Item, provider::Provider};
use prisma_hash::HashType;
use serde::{Deserialize, Serialize};

use crate::DownloadMeta;

pub struct PaperMC;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VersionList {
    versions: Vec<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BuildList {
    builds: Vec<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Url {
    downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Downloads {
    application: Application,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Application {
    name: String,
    sha256: String,
}

async fn find_version(
    version: Option<&str>,
    core_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let link = format!("https://api.papermc.io/v2/projects/{}", core_name);
    let version_list = reqwest::get(link)
        .await?
        .json::<VersionList>()
        .await?
        .versions;
    version
        .and_then(|v| version_list.iter().find(|x| x == &v))
        .or_else(|| version_list.last())
        .map(|v| v.to_owned())
        .ok_or_else(|| "No versions available".into())
}

impl PaperMC {
    pub async fn get_link(item: &Item) -> Result<DownloadMeta, Box<dyn std::error::Error>> {
        let core_name = match &item.provider {
            Provider::Core(platform) => platform.as_ref().to_lowercase(),
            _ => panic!("Used unrichable type"),
        };

        let game_version = item.version.game_version.as_deref();

        // check on exist
        let game_version = find_version(game_version, &core_name).await?;

        let version_build_link = format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}",
            core_name, game_version
        );

        let build_list = reqwest::get(version_build_link)
            .await?
            .json::<BuildList>()
            .await?
            .builds;

        match item.version.version_build.clone() {
            Some(build) => {
                if !build_list
                    .iter()
                    .any(|x| *x == build.parse::<u16>().unwrap_or_default())
                {
                    return Err(
                        format!("not found version {} with build {}", game_version, build).into(),
                    );
                }

                let (buildlink, url) = gen_link(core_name, &game_version, &build).await?;

                Ok(DownloadMeta {
                    download_link: format!(
                        "{}/downloads/{}",
                        buildlink, url.downloads.application.name
                    ),
                    hash: HashType::new_sha256(url.downloads.application.sha256),
                    game_version,
                    version_build: Some(build),
                })
            }
            None => {
                if let Some(last_build) = build_list.last().map(|x| x.to_string()) {
                    let (buildlink, url) = gen_link(core_name, &game_version, &last_build).await?;

                    Ok(DownloadMeta {
                        download_link: format!(
                            "{}/downloads/{}",
                            buildlink, url.downloads.application.name
                        ),
                        hash: HashType::new_sha256(url.downloads.application.sha256),
                        game_version,
                        version_build: Some(last_build),
                    })
                } else {
                    Err(format!("not found version {}", game_version).into())
                }
            }
        }
    }
}

async fn gen_link(
    core_name: String,
    game_version: &String,
    last_build: &str,
) -> Result<(String, Url), Box<dyn std::error::Error>> {
    let buildlink = format!(
        "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
        core_name, game_version, last_build
    );
    let url = reqwest::get(&buildlink).await?.json::<Url>().await?;
    Ok((buildlink, url))
}
