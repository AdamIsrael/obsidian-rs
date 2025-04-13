use super::community_plugin::CommunityPlugin;
use super::error::ObsidianError;
use super::utils;

use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::{create_dir_all, remove_file, File};
use std::io::copy;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize)]
pub struct ObsidianReleases {
    pub community_plugins: Vec<CommunityPlugin>,
}

impl Default for ObsidianReleases {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsidianReleases {
    pub fn new() -> Self {
        let mut s = Self {
            community_plugins: Vec::new(),
        };
        s.refresh_community_plugins().unwrap();

        s
    }
    fn get_config_path(&self) -> PathBuf {
        PathBuf::from(
            shellexpand::tilde("~/.md2ms/obsidian/")
                .to_string()
                .to_owned(),
        )
    }

    /// Refresh the community plugins list
    /// Populates the community_plugins field with the latest list of community plugins via cached file
    /// or fetches it from the internet if the cached file is not available or outdated.
    fn refresh_community_plugins(&mut self) -> Result<(), ObsidianError> {
        // Check a locally cached version of the file
        let config = self.get_config_path();

        if create_dir_all(config).is_err() {
            // Bail out if we can't create the directory
            return Err(ObsidianError::DirectoryCreationError);
        }

        let cache = self.get_config_path().join("community-plugins.json");
        if cache.exists() && cache.is_file() {
            if let Ok(file) = File::open(&cache) {
                // Checking the age of the cached filed is kinda ugly
                // TODO: Need to check that this works cross-platform.
                let seconds = file
                    .metadata()
                    .unwrap()
                    .created()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let now = UNIX_EPOCH.elapsed().unwrap().as_secs();
                let age = now - seconds;

                // For now, if the file is more than an hour old, fetch it again
                if age > 3600 {
                    let _ = remove_file(&cache);
                } else {
                    let contents = utils::slurp(&cache);

                    if let Ok(p) = serde_json::from_str(&contents) {
                        self.community_plugins = p;
                        return Ok(());
                    } else {
                        return Err(ObsidianError::ParseError);
                    }
                }
            }
        }

        // Fetch community plugins from GitHub
        let data = utils::slurp_url("https://raw.githubusercontent.com/obsidianmd/obsidian-releases/refs/heads/master/community-plugins.json".to_string());

        if let Ok(mut out) = File::create(cache) {
            if copy(&mut data.as_bytes(), &mut out).is_ok() {
                // Parse the JSON response
                let p: Vec<CommunityPlugin> = serde_json::from_str(&data).unwrap();

                // Update the community_plugins field
                self.community_plugins = p;
                return Ok(());
            }
        }

        Err(ObsidianError::OtherError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obsidian_releases_refresh_community_plugins() {
        let mut or = ObsidianReleases::new();
        let _ = or.refresh_community_plugins();
        assert!(or.community_plugins.len() > 0);
    }
}
