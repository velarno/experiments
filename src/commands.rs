use anyhow::Result;
use crate::config::Config;
use crate::git;

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
pub fn view_workspace(workspace: Option<&str>) -> Result<()> {
    let config = Config::load()?;

    if config.workspaces.is_empty() {
        println!("No workspaces configured.");
        println!("Use 'figgit new <workspace> --name <name> --email <email>' to create one.");
        return Ok(());
    }

    match workspace {
        Some(name) => {
            // View a specific workspace
            let workspace_config = config.get_workspace(name)?;
            println!("Workspace: {}", name);
            println!("  Name:  {}", workspace_config.name);
            println!("  Email: {}", workspace_config.email);
        }
        None => {
            // View all workspaces
            println!("Configured workspaces:");
            println!();

            let mut workspaces: Vec<_> = config.workspaces.iter().collect();
            workspaces.sort_by_key(|(name, _)| *name);

            for (name, workspace_config) in workspaces {
                println!("  {}:", name);
                println!("    Name:  {}", workspace_config.name);
                println!("    Email: {}", workspace_config.email);
                println!();
            }
        }
    }

    Ok(())
}

/// Show the current git configuration and compare with workspaces
pub fn status() -> Result<()> {
    let config = Config::load()?;

    println!("Git Configuration Status");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Get current local git config
    match git::get_local_config() {
        Ok((name, email)) => {
            println!("Current local git config:");
            println!("  Name:  {}", name);
            println!("  Email: {}", email);
            println!();

            // Try to find a matching workspace
            if let Some((workspace_name, _)) = config.find_matching_workspace(&name, &email) {
                println!("✓ Matches workspace: '{}'", workspace_name);
            } else {
                println!("⚠ Does not match any configured workspace");

                if !config.workspaces.is_empty() {
                    println!();
                    println!("Available workspaces:");
                    let mut workspaces: Vec<_> = config.workspaces.keys().collect();
                    workspaces.sort();
                    for workspace in workspaces {
                        println!("  - {}", workspace);
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠ Unable to read local git config: {}", e);
            println!();

            if !config.workspaces.is_empty() {
                println!("Available workspaces:");
                let mut workspaces: Vec<_> = config.workspaces.keys().collect();
                workspaces.sort();
                for workspace in workspaces {
                    println!("  - {}", workspace);
                }
                println!();
                println!("Use 'figgit use <workspace>' to apply a workspace configuration.");
            }
        }
    }

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
        config.add_workspace("test", "Test User", "test@example.com").unwrap();

        assert!(config.get_workspace("test").is_ok());
    }

    #[test]
    fn test_update_workspace_modifies_existing() {
        let mut config = Config::default();
        config.add_workspace("test", "Test User", "test@example.com").unwrap();
        config.update_workspace("test", Some("New Name"), None).unwrap();

        let workspace = config.get_workspace("test").unwrap();
        assert_eq!(workspace.name, "New Name");
        assert_eq!(workspace.email, "test@example.com");
    }

    #[test]
    fn test_delete_workspace_removes_config() {
        let mut config = Config::default();
        config.add_workspace("test", "Test User", "test@example.com").unwrap();
        config.delete_workspace("test").unwrap();

        assert!(config.get_workspace("test").is_err());
    }
}
