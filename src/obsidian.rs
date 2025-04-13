use super::community_plugin::CommunityPlugin;
use super::plugin::PluginManifest;
use super::release::ObsidianReleases;
use super::utils;

use flate2::read::GzDecoder;
use std::fs::{create_dir_all, remove_file, File};
use tar::Archive;

use serde_json;
use serde_json::Value;

use std::io::{copy, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Represents an Obsidian vault.
pub struct Obsidian {
    pub vault_path: PathBuf,
    pub config_path: PathBuf,
}

impl Obsidian {
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
        println!("Downloading manifest from {}", manifest_url);
        let manifest_string = utils::slurp_url(manifest_url);

        println!("Parsing manifest '{}'", manifest_string);
        if let Ok(manifest) = PluginManifest::from_manifest(&manifest_string) {
            // https://github.com/Taitava/obsidian-shellcommands/archive/refs/tags/0.23.0.tar.gz
            let release_url = format!(
                "{}/archive/refs/tags/{}.tar.gz",
                plugin.get_repo_url(),
                manifest.version
            );

            // Download the plugin from the given URL
            let temp = utils::get_temp_filename();

            println!(
                "Downloading plugin from {} to {}",
                release_url,
                temp.display()
            );
            if utils::download_to_file(release_url, temp.clone()).is_ok() {
                println!("Opening temp file");
                if let Ok(tar_gz) = File::open(temp) {
                    // extract the plugin to ~/plugins/<plugin_name>
                    let tar = GzDecoder::new(tar_gz);
                    let mut archive = Archive::new(tar);
                    println!("unpacking archive to new path");
                    if archive.unpack(path).is_ok() {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_community_plugins(&self) -> serde_json::Result<Vec<serde_json::Value>> {
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

    pub fn install_community_plugin(&mut self, id: String) -> bool {
        if let Ok(mut plugins) = self.get_community_plugins() {
            if let Some(plugin) = ObsidianReleases::new()
                .community_plugins
                .iter()
                .find(|p| p.id == id)
            {
                // Install the plugin
                // create the plugin folder, i.e., plugins/<plugin_name>
                let path = self.config_path.join("plugins").join(&plugin.id);
                create_dir_all(&path).unwrap();

                if self.download_plugin(plugin, path) {
                    plugins.push(serde_json::to_value(&plugin.id).unwrap());
                    // write the file
                    return self.write(plugins, self.config_path.join("community-plugins.json"));
                }
                // let url = plugin.get_repo_url();
                // let temp = get_temp_filename();
                // // download the plugin into a temporary file
                // if self.download_plugin(url, temp) {
                //     // Extract the tarball into the plugin folder
                //     // fs::rename(temp, path.join("plugin.zip")).unwrap();

                // }
            }
        }
        false
    }

    pub fn uninstall_community_plugin(&mut self, plugin: String) -> bool {
        if let Ok(mut plugins) = self.get_community_plugins() {
            // Iterate through the plugins and remove the one that matches
            let index = plugins.iter().position(|x| *x == plugin).unwrap();
            plugins.remove(index);

            // Remove the plugin from the filesystem
            // TODO: Test this
            // let plugin_path = self.config_path.join("plugins").join(plugin);
            // let _ = remove_dir_all(plugin_path);

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
        // let data = r#"
        //     [
        //       "metadata-extractor",
        //       "obsidian-advanced-uri",
        //       "obsidian-enhancing-export",
        //       "cmdr",
        //       "obsidian-shellcommands",
        //       "dataview",
        //       "templater-obsidian"
        //     ]
        // "#;
        let vault_path = PathBuf::from("./vaults/Blank");
        let mut obsidian = Obsidian::new(vault_path);

        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 0);

        // Add a plugin
        // I may not want to actually download it during unit tests?
        // I could maybe add a fake plugin into my git repo, though, so I can test the code.
        obsidian.install_community_plugin("obsidian-shellcommands".to_string());
        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 1);

        // Remove a plugin
        obsidian.uninstall_community_plugin("obsidian-shellcommands".to_string());
        let plugins = obsidian.get_community_plugins();

        assert_eq!(plugins.unwrap().len(), 0);
    }
}
