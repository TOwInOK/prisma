/// Store module provides functionality for managing game files and extensions
/// through a centralized storage system with validation and repair capabilities.
use std::{path::PathBuf, sync::Arc};

use prisma_core::{
    extension::ExtensionType, item::Item, provider::Provider, CORE_DIR, MODS_DIR, PLUGINS_DIR,
    STORE_CORES_DIR, STORE_MODS_DIR, STORE_PATH, STORE_PLUGINS_DIR, STORE_TEMP_DIR,
};
use prisma_hash::HashType;
use prisma_providers::DownloadMeta;
use ron::ser::PrettyConfig;
use tempfile::Builder;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};

/// Main store struct that holds all managed items
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Store {
    /// Vector of all store items
    pub inner: Vec<StoreItem>,
}

/// Individual store item representing a managed file
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct StoreItem {
    /// Original item metadata
    pub item: Item,
    /// File hash for validation
    pub hash: HashType,
    /// Path to file location
    pub path: String,
    /// Path to linked path
    pub symbol_link: String,
    /// Url to download
    pub url: String,
}

impl Store {
    /// Adds a new item to the store, downloading and setting up required files
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

        let sym_link_end = make_symbol_link(item, file_name, &end_path).await?;

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
            symbol_link: sym_link_end,
        });

        Ok(())
    }

    /// Updates store with new items while maintaining a backup in case of failure
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

        // add clear all symbol links if got error

        *self = backup.lock().await.clone();

        Ok(())
    }

    /// Loads store from file
    pub async fn load(
        path: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(ron::de::from_bytes(&tokio::fs::read(path).await?)?)
    }

    /// Validates all store items, returning list of invalid ones
    pub async fn validate(
        &mut self,
    ) -> Result<Vec<StoreItem>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let mut invalid_items = Vec::new();

        for item in &self.inner {
            let symbol_link_valid = check_symbol_link(item).await;
            let hash_valid = check_file_hash(item).await;

            if !symbol_link_valid || !hash_valid {
                invalid_items.push(item.clone());
            }
        }

        Ok(invalid_items)
    }

    /// Attempts to repair invalid items, returns list of items that couldn't be repaired
    pub async fn repair(
        &mut self,
    ) -> Result<Vec<StoreItem>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Get list of invalid items
        let invalid_items = self.validate().await?;
        let mut failed_repairs = Vec::new();

        for invalid_item in &invalid_items {
            // Try to repair each invalid item
            let repair_result = async {
                // Check file
                if !check_file_hash(invalid_item).await {
                    // If file is corrupted or missing, download it again
                    let (_, prefix) = get_store_item_location(&invalid_item.item);
                    let (saved_temp_path, _) =
                        download_file(&invalid_item.url, &prefix, &invalid_item.hash).await?;

                    // Move file to destination
                    tokio::fs::rename(&saved_temp_path, &invalid_item.path).await?;
                }

                // Check symbolic link
                if !check_symbol_link(invalid_item).await {
                    // If symbolic link is corrupted or missing, recreate it
                    // First remove old link if exists
                    // ignore error
                    let _ = tokio::fs::remove_file(&invalid_item.symbol_link).await;

                    // Create new symbolic link
                    tokio::fs::symlink_file(&invalid_item.path, &invalid_item.symbol_link).await?;
                }

                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            }
            .await;

            // If failed to repair item, add it to failed repairs list
            if repair_result.is_err() {
                failed_repairs.push(invalid_item.clone());
            }
        }

        Ok(failed_repairs)
    }

    /// Saves store to file
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

/// Creates a symbolic link for the given item
async fn make_symbol_link(
    item: &Item,
    file_name: String,
    end_path: &String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let sym_link_end = format!(
        "{}/{}",
        match &item.provider {
            Provider::Core(_) => CORE_DIR,
            Provider::Extension((_, _, ext)) => match ext {
                ExtensionType::Mod(_) => MODS_DIR,
                ExtensionType::Plugin(_) => PLUGINS_DIR,
            },
        },
        &file_name
    );
    tokio::fs::symlink_file(end_path, &sym_link_end).await?;
    Ok(sym_link_end)
}

/// Checks if a symbolic link is valid and exists
pub async fn check_symbol_link(item: &StoreItem) -> bool {
    if let Ok(metadata) = tokio::fs::symlink_metadata(&item.path).await {
        metadata.is_symlink()
    } else {
        false
    }
}

/// Validates file integrity by comparing its hash
pub async fn check_file_hash(item: &StoreItem) -> bool {
    if let Ok(file) = tokio::fs::read(&item.path).await.as_deref() {
        item.hash.compare(file).is_ok()
    } else {
        false
    }
}

/// Gets the storage location and prefix for an item based on its provider
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

/// Downloads a file from URL and validates its hash
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
