use prisma_core::item::Item;
use prisma_hash::HashType;
use serde::{Deserialize, Serialize};

use crate::DownloadMeta;

pub struct Purpur;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VersionList {
    versions: Vec<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BuildList {
    builds: Builds,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Builds {
    latest: String,
    all: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FileHash {
    md5: String,
}

/// https://api.purpurmc.org/v2/purpur/{Version}/{Build}/download
const MAIN_LINK: &str = "https://api.purpurmc.org/v2/purpur";

/// Find version in version list, if exist give out version or give error
async fn find_version(version: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let version_list = reqwest::get(MAIN_LINK)
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

impl Purpur {
    pub async fn get_link(item: &Item) -> Result<DownloadMeta, Box<dyn std::error::Error>> {
        let version = find_version(item.version.game_version.as_deref()).await?;
        //Version string
        let verlink = format!("{}/{}", MAIN_LINK, version);
        let build_list = reqwest::get(verlink).await?.json::<BuildList>().await?;
        let build_list_latest = build_list.builds.latest;
        let build_list = build_list.builds.all;

        match item.version.version_build.as_ref() {
            Some(local_build) => {
                if build_list.iter().any(|x| x == local_build) {
                    let (build_link, file_hash) = gen_link(&version, local_build).await?;

                    Ok(DownloadMeta {
                        download_link: format!("{}/download", build_link),
                        hash: HashType::new_md5(file_hash.md5),
                        game_version: version,
                        version_build: Some(local_build.clone()),
                    })
                } else {
                    Err(format!("not found version {} with build {}", version, local_build).into())
                }
            }
            None => {
                let (build_link, file_hash) = gen_link(&version, &build_list_latest).await?;

                Ok(DownloadMeta {
                    download_link: format!("{}/download", build_link),
                    hash: HashType::new_md5(file_hash.md5),
                    game_version: version,
                    version_build: Some(build_list_latest),
                })
            }
        }
    }
}

async fn gen_link(
    version: &str,
    local_build: &str,
) -> Result<(String, FileHash), Box<dyn std::error::Error>> {
    let build_link = format!("{}/{}/{}", MAIN_LINK, version, &local_build);
    let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;
    Ok((build_link, file_hash))
}
