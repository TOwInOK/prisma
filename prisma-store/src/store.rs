use std::{path::PathBuf, sync::Arc};

use prisma_core::{
    extension::ExtensionType, item::Item, provider::Provider, STORE_CORES_DIR, STORE_MODS_DIR,
    STORE_PATH, STORE_PLUGINS_DIR, STORE_TEMP_DIR,
};
use prisma_hash::HashType;
use prisma_providers::DownloadMeta;
use ron::ser::PrettyConfig;
use tempfile::Builder;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Store {
    pub inner: Vec<StoreItem>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct StoreItem {
    pub item: Item,
    pub hash: HashType,
    pub path: String,
    pub url: String,
}

impl Store {
    pub async fn push(
        &mut self,
        item: &Item,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Fetch meta
        let meta = DownloadMeta::fetch(item).await?;

        // Get store location and prefix
        let (end_location, prefix) = get_store_item_location(item);

        // Download file
        let (saved_temp_path, file_name) =
            download_file(&meta.download_link, &prefix, &meta.hash).await?;

        // End path for item
        let end_path = format!("{}/{}.jar", end_location, &file_name);

        // Move file to final location
        tokio::fs::rename(&saved_temp_path, &end_path).await?;

        // Push to store
        self.inner.push(StoreItem {
            item: item.clone().with_version(prisma_core::version::Version {
                game_version: Some(meta.game_version),
                version_build: meta.version_build,
                channel: item.version.channel,
            }),
            hash: meta.hash,
            path: end_path,
            url: meta.download_link,
        });

        Ok(())
    }

    pub async fn fill_new(
        &mut self,
        items: Vec<&Item>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Create backup of current store
        let backup = Arc::new(Mutex::new(self.clone()));

        // Try to update store
        let handles = items
            .into_iter()
            .filter(|item| !item.options.freeze || item.options.force_update)
            .cloned()
            .map(|item| {
                let backup_clone = backup.clone();
                tokio::spawn(async move {
                    let mut store = backup_clone.lock().await;
                    store.push(&item).await
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.await??;
        }

        // Save updated store
        self.save(STORE_PATH).await?;

        *self = backup.lock().await.clone();

        Ok(())
    }

    pub async fn load(
        path: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(ron::de::from_bytes(&tokio::fs::read(path).await?)?)
    }

    pub async fn save(
        &self,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        tokio::fs::write(
            path,
            ron::ser::to_string_pretty(&self, PrettyConfig::default().enumerate_arrays(true))?,
        )
        .await?;
        Ok(())
    }
}

fn get_store_item_location(item: &Item) -> (&'static str, String) {
    match &item.provider {
        Provider::Core(platform) => (STORE_CORES_DIR, format!("{}-{}-", &item.provider, platform)),
        Provider::Extension((_, _, ext)) => (
            match ext {
                ExtensionType::Mod(_) => STORE_MODS_DIR,
                ExtensionType::Plugin(_) => STORE_PLUGINS_DIR,
            },
            format!("{}-{}-", &item.provider, ext),
        ),
    }
}

async fn download_file(
    url: &str,
    prefix: &str,
    expected_hash: &HashType,
) -> Result<(PathBuf, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let response = reqwest::get(url).await?;

    let temp_dir = Builder::new().prefix(prefix).tempdir_in(STORE_TEMP_DIR)?;

    let file_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .ok_or_else(|| format!("Invalid path in URL: {:#?}", response.url()))?
        .to_string();

    let body = response.bytes().await?;
    expected_hash.compare(&body)?;

    let file_path = temp_dir.path().join(&file_name);
    let mut file = File::create_new(&file_path).await?;

    file.write_all(&body).await?;

    Ok((file_path, file_name))
}
