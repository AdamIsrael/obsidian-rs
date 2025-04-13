use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(alias = "minAppVersion")]
    pub min_app_version: String,
    pub description: String,
    pub author: String,
    #[serde(alias = "authorUrl")]
    pub author_url: String,
    #[serde(alias = "fundingUrl")]
    pub funding_url: String,
    #[serde(alias = "isDesktopOnly")]
    pub is_desktop_only: bool,
}

impl PluginManifest {
    pub fn from_manifest(manifest: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manifest() {
        let data = r#"
            {
           	"id": "obsidian-shellcommands",
           	"name": "Shell commands",
           	"version": "0.23.0",
           	"minAppVersion": "1.4.0",
           	"description": "You can predefine system commands that you want to run frequently, and assign hotkeys for them. For example open external applications. Automatic execution is also supported, and execution via URI links.",
           	"author": "Jarkko Linnanvirta",
           	"authorUrl": "https://github.com/Taitava",
            "fundingUrl": "https://publish.obsidian.md/shellcommands/Donate",
           	"isDesktopOnly": true
            }"#;

        let pm = PluginManifest::from_manifest(data).unwrap();
        assert_eq!(pm.id, "obsidian-shellcommands");
        assert_eq!(pm.name, "Shell commands");
        assert_eq!(pm.version, "0.23.0");
        assert_eq!(pm.min_app_version, "1.4.0");
        assert_eq!(
            pm.funding_url,
            "https://publish.obsidian.md/shellcommands/Donate"
        );
        assert_eq!(pm.is_desktop_only, true);
    }
}
