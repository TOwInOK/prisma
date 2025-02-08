use prisma_core::{item::Item, version::Version};
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
    match version {
        None => match version_list.last() {
            Some(e) => Ok(e.to_owned()),
            None => Err("Not found latest version".into()),
        },
        Some(version) => {
            if version_list.contains(&version.to_string()) {
                Ok(version.to_owned())
            } else {
                Err(format!("Version {} not found", version).into())
            }
        }
    }
}

impl Purpur {
    pub async fn get_link(item: &Item) -> Result<DownloadMeta, Box<dyn std::error::Error>> {
        let (version, build) = match &item.version {
            Version::Latest => (None, None),
            Version::Specific(version, build, _) => (version.clone(), build.clone()),
        };

        let version = find_version(version.as_deref()).await?;
        //Version string
        let verlink = format!("{}/{}", MAIN_LINK, version);
        let build_list = reqwest::get(verlink).await?.json::<BuildList>().await?;
        let build_list_latest = build_list.builds.latest;
        let build_list = build_list.builds.all;

        match build {
            Some(ref local_build) => {
                if build_list.iter().any(|x| x == local_build) {
                    let build_link = format!("{}/{}/{}", MAIN_LINK, version, &local_build);
                    let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;

                    Ok(DownloadMeta {
                        download_link: format!("{}/download", build_link),
                        hash: HashType::new_md5(file_hash.md5),
                        version,
                        build,
                    })
                } else {
                    Err(format!("not found version {} with build {}", version, local_build).into())
                }
            }
            None => {
                let build_link = format!("{}/{}/{}", MAIN_LINK, version, &build_list_latest);

                Ok(DownloadMeta {
                    download_link: format!("{}/download", build_link),
                    hash: HashType::new_md5(
                        reqwest::get(&build_link)
                            .await?
                            .json::<FileHash>()
                            .await?
                            .md5,
                    ),
                    version,
                    build: Some(build_list_latest),
                })
            }
        }
    }
}
