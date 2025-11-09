use crate::config::Config;
use crate::git;
use crate::output::{self, OutputFormat, StatusOutput};
use anyhow::Result;

/// Create a new workspace configuration
pub fn new_workspace(workspace: &str, name: &str, email: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.add_workspace(workspace, name, email)?;
    config.save()?;

    println!("✓ Created workspace '{}'", workspace);
    println!("  Name:  {}", name);
    println!("  Email: {}", email);

    Ok(())
}

/// Update an existing workspace configuration
pub fn update_workspace(workspace: &str, name: Option<&str>, email: Option<&str>) -> Result<()> {
    if name.is_none() && email.is_none() {
        println!("No changes specified. Use --name and/or --email to update the workspace.");
        return Ok(());
    }

    let mut config = Config::load()?;
    config.update_workspace(workspace, name, email)?;
    config.save()?;

    println!("✓ Updated workspace '{}'", workspace);
    let updated = config.get_workspace(workspace)?;
    println!("  Name:  {}", updated.name);
    println!("  Email: {}", updated.email);

    Ok(())
}

/// Apply a workspace configuration to the local git repository
pub fn use_workspace(workspace: &str) -> Result<()> {
    let config = Config::load()?;
    let workspace_config = config.get_workspace(workspace)?;

    git::set_local_config(&workspace_config.name, &workspace_config.email)?;

    println!("✓ Applied workspace '{}' to local git config", workspace);
    println!("  Name:  {}", workspace_config.name);
    println!("  Email: {}", workspace_config.email);

    Ok(())
}

/// View workspace configurations
pub fn view_workspace(workspace: Option<&str>, format: OutputFormat) -> Result<()> {
    let config = Config::load()?;

    match workspace {
        Some(name) => {
            // View a specific workspace
            let workspace_config = config.get_workspace(name)?;

            match format {
                OutputFormat::Json => {
                    let output = serde_json::json!({
                        "name": name,
                        "user_name": workspace_config.name,
                        "email": workspace_config.email
                    });
                    println!("{}", serde_json::to_string_pretty(&output).unwrap());
                }
                _ => {
                    println!("Workspace: {}", name);
                    println!("  Name:  {}", workspace_config.name);
                    println!("  Email: {}", workspace_config.email);
                }
            }
        }
        None => {
            // View all workspaces
            output::print_workspaces(&config.workspaces, format);
        }
    }

    Ok(())
}

/// List all workspace configurations
pub fn list_workspaces(format: OutputFormat) -> Result<()> {
    let config = Config::load()?;
    output::print_workspaces(&config.workspaces, format);
    Ok(())
}

/// Show the current git configuration and compare with workspaces
pub fn status(format: OutputFormat) -> Result<()> {
    let config = Config::load()?;

    let mut status_output = StatusOutput {
        current_name: None,
        current_email: None,
        matching_workspace: None,
        available_workspaces: config.workspaces.keys().cloned().collect(),
    };

    status_output.available_workspaces.sort();

    // Get current local git config
    if let Ok((name, email)) = git::get_local_config() {
        status_output.current_name = Some(name.clone());
        status_output.current_email = Some(email.clone());

        // Try to find a matching workspace
        if let Some((workspace_name, _)) = config.find_matching_workspace(&name, &email) {
            status_output.matching_workspace = Some(workspace_name.clone());
        }
    }

    output::print_status(&status_output, format);

    Ok(())
}

/// Delete a workspace configuration
pub fn delete_workspace(workspace: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.delete_workspace(workspace)?;
    config.save()?;

    println!("✓ Deleted workspace '{}'", workspace);

    Ok(())
}

/// Import a workspace configuration from git config
pub fn import_workspace(workspace: &str, global: bool, from: Option<&str>) -> Result<()> {
    let mut config = Config::load()?;

    let (name, email) = if let Some(repo_path) = from {
        // Import from a specific repository
        println!("Importing from repository: {}", repo_path);
        git::get_config_from_repo(repo_path)?
    } else if global {
        // Import from global config
        println!("Importing from global git config");
        git::get_global_config()?
    } else {
        // Import from local config (default)
        println!("Importing from local git config");
        git::get_local_config()?
    };

    config.add_workspace(workspace, &name, &email)?;
    config.save()?;

    println!("✓ Imported workspace '{}'", workspace);
    println!("  Name:  {}", name);
    println!("  Email: {}", email);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests are simplified and would need more sophisticated
    // setup/teardown in a real testing environment to avoid conflicts
    // with the actual user config.

    #[test]
    fn test_new_workspace_creates_config() {
        // This is a basic test structure
        // In a real scenario, we'd need to mock the config path
        let mut config = Config::default();
        config
            .add_workspace("test", "Test User", "test@example.com")
            .unwrap();

        assert!(config.get_workspace("test").is_ok());
    }

    #[test]
    fn test_update_workspace_modifies_existing() {
        let mut config = Config::default();
        config
            .add_workspace("test", "Test User", "test@example.com")
            .unwrap();
        config
            .update_workspace("test", Some("New Name"), None)
            .unwrap();

        let workspace = config.get_workspace("test").unwrap();
        assert_eq!(workspace.name, "New Name");
        assert_eq!(workspace.email, "test@example.com");
    }

    #[test]
    fn test_delete_workspace_removes_config() {
        let mut config = Config::default();
        config
            .add_workspace("test", "Test User", "test@example.com")
            .unwrap();
        config.delete_workspace("test").unwrap();

        assert!(config.get_workspace("test").is_err());
    }

    #[test]
    fn test_import_workspace_from_global_config() {
        // Test that we can attempt to get global config
        // This is environment-dependent
        let result = git::get_global_config();

        if result.is_ok() {
            let (name, email) = result.unwrap();
            assert!(!name.is_empty(), "Global user.name should not be empty");
            assert!(!email.is_empty(), "Global user.email should not be empty");

            // Verify we can create a workspace with these values
            let mut config = Config::default();
            config.add_workspace("imported", &name, &email).unwrap();

            let workspace = config.get_workspace("imported").unwrap();
            assert_eq!(workspace.name, name);
            assert_eq!(workspace.email, email);
        }
    }

    #[test]
    fn test_import_workspace_from_local_config() {
        // Test that we can attempt to get local config
        // This is environment-dependent
        if git::is_git_repo() {
            let result = git::get_local_config();

            if result.is_ok() {
                let (name, email) = result.unwrap();
                assert!(!name.is_empty(), "Local user.name should not be empty");
                assert!(!email.is_empty(), "Local user.email should not be empty");

                // Verify we can create a workspace with these values
                let mut config = Config::default();
                config
                    .add_workspace("imported-local", &name, &email)
                    .unwrap();

                let workspace = config.get_workspace("imported-local").unwrap();
                assert_eq!(workspace.name, name);
                assert_eq!(workspace.email, email);
            }
        }
    }

    #[test]
    fn test_import_workspace_from_invalid_repo() {
        // Test that importing from an invalid repo fails gracefully
        let result = git::get_config_from_repo("/nonexistent/path");
        assert!(result.is_err(), "Should fail with invalid repository path");
    }

    #[test]
    fn test_import_workspace_duplicate_name() {
        // Test that importing with a duplicate workspace name fails
        let mut config = Config::default();
        config
            .add_workspace("existing", "Existing User", "existing@example.com")
            .unwrap();

        // Attempting to add another workspace with the same name should fail
        let result = config.add_workspace("existing", "New User", "new@example.com");
        assert!(
            result.is_err(),
            "Should fail when workspace name already exists"
        );
    }

    #[test]
    fn test_list_workspaces_empty() {
        // Test listing when there are no workspaces
        let config = Config::default();
        assert!(config.workspaces.is_empty());
    }

    #[test]
    fn test_list_workspaces_with_data() {
        // Test listing workspaces
        let mut config = Config::default();
        config
            .add_workspace("work", "Work User", "work@example.com")
            .unwrap();
        config
            .add_workspace("personal", "Personal User", "personal@example.com")
            .unwrap();

        assert_eq!(config.workspaces.len(), 2);
        assert!(config.get_workspace("work").is_ok());
        assert!(config.get_workspace("personal").is_ok());
    }
}
