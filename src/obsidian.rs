use super::community_plugin::CommunityPlugin;
use super::plugin::PluginManifest;
use super::release::ObsidianReleases;
use super::utils;

use std::fs::{create_dir_all, remove_file, File};

use serde_json::Value;

use std::fs::remove_dir_all;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const PLUGIN_FILES: &[&str] = &["main.js", "manifest.json", "styles.css"];

/// Represents an Obsidian vault.
pub struct Obsidian {
    pub vault_path: PathBuf,
    pub config_path: PathBuf,
}

impl Obsidian {
    // Opens an Obsidian vault from the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let vault_path: PathBuf = path.as_ref().to_string_lossy().into_owned().into();
        let config_path = vault_path.join(".obsidian");

        Obsidian {
            vault_path,
            config_path,
        }
    }

    pub fn is_vault(&self) -> bool {
        self.config_path.is_dir() && self.config_path.ends_with(".obsidian")
    }

    fn download_plugin(&self, plugin: &CommunityPlugin, path: PathBuf) -> bool {
        let manifest_url = plugin.get_manifest_url();

        // Download and parse the plugin's manifest
        let manifest_string = utils::slurp_url(manifest_url);

        if let Ok(manifest) = PluginManifest::from_manifest(&manifest_string) {
            // To install a plugin, we need to download the following files from the plugin's latest release:
            // - main.js
            // - manifest.json
            // - style.css (if it exists)
            let required_files = 2;
            let mut found_files = 0;
            for file in PLUGIN_FILES {
                // Download the file
                let release_url = format!(
                    "{}/releases/download/{}/{}",
                    plugin.get_repo_url(),
                    manifest.version,
                    file
                );

                let fp = format!("{}/{}", path.display(), file);
                if utils::download_to_file(release_url, PathBuf::from(fp)).is_ok() {
                    found_files += 1;
                }
            }
            if found_files >= required_files {
                return true;
            }
        }
        false
    }

    /// Get the installed community plugins
    pub fn get_installed_community_plugins(&self) -> serde_json::Result<Vec<serde_json::Value>> {
        let path = self.config_path.join("community-plugins.json");
        if utils::file_exists(&path) {
            let contents = utils::slurp(&path);
            if !contents.is_empty() {
                let json: Value = serde_json::from_str(&contents).unwrap();
                return Ok(json.as_array().unwrap().clone());
            }
        }
        Ok(vec![])
    }

    /// Install a community plugin by id
    pub fn install_community_plugin(&mut self, id: String) -> bool {
        if let Ok(mut plugins) = self.get_installed_community_plugins() {
            // Check the latest community plugins released and look for this plugin
            if let Some(plugin) = ObsidianReleases::new()
                .community_plugins
                .iter()
                .find(|p| p.id == id)
            {
                // create the plugin folder, i.e., plugins/<plugin_name>
                let path = self.config_path.join("plugins").join(&plugin.id);
                create_dir_all(&path).unwrap();

                if self.download_plugin(plugin, path) {
                    // We downloaded the plugin successfully, so enable it.
                    plugins.push(serde_json::to_value(&plugin.id).unwrap());
                    // write the file
                    return self.write(plugins, self.config_path.join("community-plugins.json"));
                }
            }
        }
        false
    }

    /// Uninstall a community plugin by id
    pub fn uninstall_community_plugin(&mut self, plugin: String) -> bool {
        if let Ok(mut plugins) = self.get_installed_community_plugins() {
            // Iterate through the plugins and remove the one that matches
            let index = plugins.iter().position(|x| *x == plugin).unwrap();
            plugins.remove(index);

            // Remove the plugin from the filesystem
            let plugin_path = self.config_path.join("plugins").join(plugin);
            let _ = remove_dir_all(plugin_path);

            let path = self.config_path.join("community-plugins.json");
            if plugins.is_empty() {
                let _ = remove_file(path);
                return true;
            } else {
                // write the file
                return self.write(plugins, path);
            }
        }
        false
    }

    fn write(&mut self, values: Vec<Value>, path: PathBuf) -> bool {
        if let Ok(file) = File::create(path) {
            let mut writer = BufWriter::new(file);
            let _ = serde_json::to_writer(&mut writer, &values);
            let _ = writer.flush();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vault_path = PathBuf::from("./vaults/Blank");
        let obsidian = Obsidian::new(vault_path);

        assert_eq!(obsidian.vault_path, PathBuf::from("./vaults/Blank"));
        assert_eq!(
            obsidian.config_path,
            PathBuf::from("./vaults/Blank/.obsidian")
        );
    }

    #[test]
    fn test_is_vault() {
        let vault_path = PathBuf::from("./vaults/Blank");
        let obsidian = Obsidian::new(vault_path);

        assert!(obsidian.is_vault());
    }

    #[test]
    fn test_community_plugins() {
        let vault_path = PathBuf::from("./vaults/Blank");
        let mut obsidian = Obsidian::new(vault_path);

        let plugins = obsidian.get_installed_community_plugins();

        assert_eq!(plugins.unwrap().len(), 0);

        // Add a plugin
        // I may not want to actually download it during unit tests?
        // I could maybe add a fake plugin into my git repo, though, so I can test the code.
        obsidian.install_community_plugin("obsidian-shellcommands".to_string());
        let plugins = obsidian.get_installed_community_plugins();

        assert_eq!(plugins.unwrap().len(), 1);

        // Remove a plugin
        obsidian.uninstall_community_plugin("obsidian-shellcommands".to_string());
        let plugins = obsidian.get_installed_community_plugins();

        assert_eq!(plugins.unwrap().len(), 0);
    }
}
