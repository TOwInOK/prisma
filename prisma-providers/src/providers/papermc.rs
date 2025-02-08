use prisma_core::{item::Item, provider::Provider, version::Version};
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
    match version {
        None => match version_list.last() {
            Some(e) => Ok(e.to_owned()),
            None => Err("Not found latest version".into()),
        },
        Some(version) => {
            if version_list.iter().any(|x| x == version) {
                Ok(version.to_owned())
            } else {
                Err(format!("Version {} not found", version).into())
            }
        }
    }
}

impl PaperMC {
    pub async fn get_link(item: &Item) -> Result<DownloadMeta, Box<dyn std::error::Error>> {
        let core_name = match &item.provider {
            Provider::Core(platform) => platform.as_ref().to_lowercase(),
            _ => panic!("Used unrichable type"),
        };

        let (version, build) = match &item.version {
            Version::Latest => (None, None),
            Version::Specific(version, build, _) => (version.clone(), build.clone()),
        };

        let version = find_version(version.as_deref(), &core_name).await?;

        let verlink = format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}",
            core_name, version
        );

        let build_list = reqwest::get(verlink)
            .await?
            .json::<BuildList>()
            .await?
            .builds;

        let result = match build.as_deref() {
            Some(local_build) => {
                if build_list
                    .iter()
                    .any(|x| *x == local_build.parse::<u16>().unwrap_or_default())
                {
                    let link = format!(
                        "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
                        core_name, version, local_build
                    );
                    let url = reqwest::get(&link).await?.json::<Url>().await?;
                    Ok(DownloadMeta {
                        download_link: format!(
                            "{}/downloads/{}",
                            link, url.downloads.application.name
                        ),
                        hash: HashType::new_sha256(url.downloads.application.sha256),
                        version,
                        build,
                    })
                } else {
                    Err(format!("not found version {} with build {}", version, local_build).into())
                }
            }
            None => {
                if let Some(last_build) = build_list.last() {
                    let buildlink = format!(
                        "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
                        core_name, version, last_build
                    );
                    let url: Url = reqwest::get(&buildlink).await?.json().await?;

                    Ok(DownloadMeta {
                        download_link: format!(
                            "{}/downloads/{}",
                            buildlink, url.downloads.application.name
                        ),
                        hash: HashType::new_sha256(url.downloads.application.sha256),
                        version,
                        build,
                    })
                } else {
                    Err(format!("not found version {} with build {:#?}", version, build).into())
                }
            }
        };
        result
    }
}
