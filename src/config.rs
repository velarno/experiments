use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceConfig {
    pub name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub workspaces: HashMap<String, WorkspaceConfig>,
}

impl Config {
    /// Get the path to the config file
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Unable to determine home directory")?;
        Ok(home.join(".config").join("figgit").join("config.toml"))
    }

    /// Load the configuration from the config file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    /// Save the configuration to the config file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    /// Add a new workspace
    pub fn add_workspace(&mut self, name: &str, user_name: &str, email: &str) -> Result<()> {
        if self.workspaces.contains_key(name) {
            bail!(
                "Workspace '{}' already exists. Use 'update' to modify it.",
                name
            );
        }

        self.workspaces.insert(
            name.to_string(),
            WorkspaceConfig {
                name: user_name.to_string(),
                email: email.to_string(),
                patterns: Vec::new(),
            },
        );

        Ok(())
    }

    /// Update an existing workspace
    pub fn update_workspace(
        &mut self,
        name: &str,
        user_name: Option<&str>,
        email: Option<&str>,
    ) -> Result<()> {
        let workspace = self
            .workspaces
            .get_mut(name)
            .context(format!("Workspace '{}' not found", name))?;

        if let Some(user_name) = user_name {
            workspace.name = user_name.to_string();
        }

        if let Some(email) = email {
            workspace.email = email.to_string();
        }

        Ok(())
    }

    /// Update workspace URL patterns
    pub fn update_workspace_patterns(
        &mut self,
        name: &str,
        patterns: Vec<String>,
        reset: bool,
    ) -> Result<()> {
        let workspace = self
            .workspaces
            .get_mut(name)
            .context(format!("Workspace '{}' not found", name))?;

        if reset {
            // Replace entire patterns list
            workspace.patterns = patterns;
        } else {
            // Append to existing patterns (avoiding duplicates)
            for pattern in patterns {
                if !workspace.patterns.contains(&pattern) {
                    workspace.patterns.push(pattern);
                }
            }
        }

        Ok(())
    }

    /// Get a workspace by name
    pub fn get_workspace(&self, name: &str) -> Result<&WorkspaceConfig> {
        self.workspaces
            .get(name)
            .context(format!("Workspace '{}' not found", name))
    }

    /// Delete a workspace
    pub fn delete_workspace(&mut self, name: &str) -> Result<()> {
        if self.workspaces.remove(name).is_none() {
            bail!("Workspace '{}' not found", name);
        }
        Ok(())
    }

    /// Find a workspace that matches the given name and email
    pub fn find_matching_workspace(
        &self,
        name: &str,
        email: &str,
    ) -> Option<(&String, &WorkspaceConfig)> {
        self.workspaces
            .iter()
            .find(|(_, config)| config.name == name && config.email == email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_workspace() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        assert_eq!(config.workspaces.len(), 1);
        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.name, "John Doe");
        assert_eq!(workspace.email, "john@work.com");
    }

    #[test]
    fn test_add_duplicate_workspace() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        let result = config.add_workspace("work", "Jane Doe", "jane@work.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_workspace() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        config
            .update_workspace("work", Some("Jane Doe"), None)
            .unwrap();
        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.name, "Jane Doe");
        assert_eq!(workspace.email, "john@work.com");

        config
            .update_workspace("work", None, Some("jane@work.com"))
            .unwrap();
        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.name, "Jane Doe");
        assert_eq!(workspace.email, "jane@work.com");
    }

    #[test]
    fn test_update_nonexistent_workspace() {
        let mut config = Config::default();
        let result = config.update_workspace("work", Some("John Doe"), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_workspace() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        config.delete_workspace("work").unwrap();
        assert_eq!(config.workspaces.len(), 0);
    }

    #[test]
    fn test_delete_nonexistent_workspace() {
        let mut config = Config::default();
        let result = config.delete_workspace("work");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_matching_workspace() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();
        config
            .add_workspace("personal", "John Smith", "john@personal.com")
            .unwrap();

        let result = config.find_matching_workspace("John Doe", "john@work.com");
        assert!(result.is_some());
        let (name, _) = result.unwrap();
        assert_eq!(name, "work");

        let result = config.find_matching_workspace("Unknown", "unknown@example.com");
        assert!(result.is_none());
    }

    #[test]
    fn test_serialization() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(config.workspaces.len(), deserialized.workspaces.len());
        let workspace = deserialized.get_workspace("work").unwrap();
        assert_eq!(workspace.name, "John Doe");
        assert_eq!(workspace.email, "john@work.com");
    }

    #[test]
    fn test_update_workspace_patterns_append() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        // Append patterns
        config
            .update_workspace_patterns("work", vec!["github.com/company/*".to_string()], false)
            .unwrap();

        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.patterns.len(), 1);
        assert_eq!(workspace.patterns[0], "github.com/company/*");

        // Append more patterns
        config
            .update_workspace_patterns(
                "work",
                vec![
                    "gitlab.company.com/*".to_string(),
                    "bitbucket.org/company/*".to_string(),
                ],
                false,
            )
            .unwrap();

        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.patterns.len(), 3);
        assert!(workspace
            .patterns
            .contains(&"github.com/company/*".to_string()));
        assert!(workspace
            .patterns
            .contains(&"gitlab.company.com/*".to_string()));
        assert!(workspace
            .patterns
            .contains(&"bitbucket.org/company/*".to_string()));
    }

    #[test]
    fn test_update_workspace_patterns_no_duplicates() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        // Add pattern twice
        config
            .update_workspace_patterns("work", vec!["github.com/company/*".to_string()], false)
            .unwrap();
        config
            .update_workspace_patterns("work", vec!["github.com/company/*".to_string()], false)
            .unwrap();

        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.patterns.len(), 1);
    }

    #[test]
    fn test_update_workspace_patterns_reset() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();

        // Add initial patterns
        config
            .update_workspace_patterns(
                "work",
                vec![
                    "github.com/old/*".to_string(),
                    "gitlab.com/old/*".to_string(),
                ],
                false,
            )
            .unwrap();

        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.patterns.len(), 2);

        // Reset with new patterns
        config
            .update_workspace_patterns("work", vec!["github.com/new/*".to_string()], true)
            .unwrap();

        let workspace = config.get_workspace("work").unwrap();
        assert_eq!(workspace.patterns.len(), 1);
        assert_eq!(workspace.patterns[0], "github.com/new/*");
    }

    #[test]
    fn test_patterns_serialization() {
        let mut config = Config::default();
        config
            .add_workspace("work", "John Doe", "john@work.com")
            .unwrap();
        config
            .update_workspace_patterns("work", vec!["github.com/company/*".to_string()], false)
            .unwrap();

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        let workspace = deserialized.get_workspace("work").unwrap();
        assert_eq!(workspace.patterns.len(), 1);
        assert_eq!(workspace.patterns[0], "github.com/company/*");
    }
}
