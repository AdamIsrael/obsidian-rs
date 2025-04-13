use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommunityPlugin {
    pub id: String,
    pub author: String,
    pub name: String,
    pub description: String,
    pub repo: String,
}

impl CommunityPlugin {
    pub fn get_repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }
    pub fn get_manifest_url(&self) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/refs/heads/main/manifest.json",
            self.repo
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obsidian_release_community_plugins() {
        let data = r#"
            [
            {
                "id": "nldates-obsidian",
                "name": "Natural Language Dates",
                "author": "Argentina Ortega Sainz",
                "description": "Create date-links based on natural language.",
                "repo": "argenos/nldates-obsidian"
            },
            {
                "id": "hotkeysplus-obsidian",
                "name": "Hotkeys++",
                "author": "Argentina Ortega Sainz",
                "description": "Additional hotkeys to do common things.",
                "repo": "argenos/hotkeysplus-obsidian"
            },
            {
                "id": "obsidian-advanced-uri",
                "name": "Obsidian Advanced URI",
                "author": "Argentina Ortega Sainz",
                "description": "Advanced URI support for Obsidian.",
                "repo": "argenos/obsidian-advanced-uri"
            },
            {
                "id": "obsidian-enhancing-export",
                "name": "Obsidian Enhancing Export",
                "author": "Argentina Ortega Sainz",
                "description": "Enhancing export for Obsidian.",
                "repo": "argenos/obsidian-enhancing-export"
            },
            {
                "id": "cmdr",
                "name": "CMDR",
                "author": "Argentina Ortega Sainz",
                "description": "Command line interface for Obsidian.",
                "repo": "argenos/cmdr"
            },
            {
                "id": "obsidian-shellcommands",
                "name": "Obsidian Shell Commands",
                "author": "Argentina Ortega Sainz",
                "description": "Shell commands for Obsidian.",
                "repo": "argenos/obsidian-shellcommands"
            },
            {
                "id": "dataview",
                "name": "DataView",
                "author": "Argentina Ortega Sainz",
                "description": "DataView for Obsidian.",
                "repo": "argenos/dataview"
            },
            {
                "id": "templater-obsidian",
                "name": "Templater Obsidian",
                "author": "Argentina Ortega Sainz",
                "description": "Templater for Obsidian.",
                "repo": "argenos/templater-obsidian"
            }
            ]"#;

        let p: Vec<CommunityPlugin> = serde_json::from_str(data).unwrap();
        assert_eq!(p.len(), 8);
        assert_eq!(p[5].id, "obsidian-shellcommands");
        assert_eq!(p[5].name, "Obsidian Shell Commands");
        assert_eq!(p[5].author, "Argentina Ortega Sainz");
        assert_eq!(p[5].description, "Shell commands for Obsidian.");
        assert_eq!(p[5].repo, "argenos/obsidian-shellcommands");
    }
}
