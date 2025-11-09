use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Check if the current directory is a git repository
pub fn is_git_repo() -> bool {
    Path::new(".git").exists()
}

/// Get the current git user.name from local config
pub fn get_local_user_name() -> Result<String> {
    if !is_git_repo() {
        bail!("Not in a git repository");
    }

    let output = Command::new("git")
        .args(["config", "--local", "user.name"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("No local user.name configured");
    }

    let name = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    if name.is_empty() {
        bail!("No local user.name configured");
    }

    Ok(name)
}

/// Get the current git user.email from local config
pub fn get_local_user_email() -> Result<String> {
    if !is_git_repo() {
        bail!("Not in a git repository");
    }

    let output = Command::new("git")
        .args(["config", "--local", "user.email"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("No local user.email configured");
    }

    let email = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    if email.is_empty() {
        bail!("No local user.email configured");
    }

    Ok(email)
}

/// Set the local git user.name
pub fn set_local_user_name(name: &str) -> Result<()> {
    if !is_git_repo() {
        bail!("Not in a git repository");
    }

    let status = Command::new("git")
        .args(["config", "--local", "user.name", name])
        .status()
        .context("Failed to execute git command")?;

    if !status.success() {
        bail!("Failed to set git user.name");
    }

    Ok(())
}

/// Set the local git user.email
pub fn set_local_user_email(email: &str) -> Result<()> {
    if !is_git_repo() {
        bail!("Not in a git repository");
    }

    let status = Command::new("git")
        .args(["config", "--local", "user.email", email])
        .status()
        .context("Failed to execute git command")?;

    if !status.success() {
        bail!("Failed to set git user.email");
    }

    Ok(())
}

/// Get both user.name and user.email from local config
pub fn get_local_config() -> Result<(String, String)> {
    let name = get_local_user_name()?;
    let email = get_local_user_email()?;
    Ok((name, email))
}

/// Set both user.name and user.email in local config
pub fn set_local_config(name: &str, email: &str) -> Result<()> {
    set_local_user_name(name)?;
    set_local_user_email(email)?;
    Ok(())
}

/// Get the git user.name from global config
pub fn get_global_user_name() -> Result<String> {
    let output = Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("No global user.name configured");
    }

    let name = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    if name.is_empty() {
        bail!("No global user.name configured");
    }

    Ok(name)
}

/// Get the git user.email from global config
pub fn get_global_user_email() -> Result<String> {
    let output = Command::new("git")
        .args(["config", "--global", "user.email"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("No global user.email configured");
    }

    let email = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    if email.is_empty() {
        bail!("No global user.email configured");
    }

    Ok(email)
}

/// Get both user.name and user.email from global config
pub fn get_global_config() -> Result<(String, String)> {
    let name = get_global_user_name()?;
    let email = get_global_user_email()?;
    Ok((name, email))
}

/// Get user.name from a specific repository
pub fn get_user_name_from_repo(repo_path: &str) -> Result<String> {
    let repo_path = Path::new(repo_path);

    if !repo_path.exists() {
        bail!("Repository path does not exist: {}", repo_path.display());
    }

    if !repo_path.is_dir() {
        bail!("Repository path is not a directory: {}", repo_path.display());
    }

    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        bail!("Not a git repository: {}", repo_path.display());
    }

    let output = Command::new("git")
        .args(["-C", repo_path.to_str().unwrap(), "config", "--local", "user.name"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("No local user.name configured in repository: {}", repo_path.display());
    }

    let name = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    if name.is_empty() {
        bail!("No local user.name configured in repository: {}", repo_path.display());
    }

    Ok(name)
}

/// Get user.email from a specific repository
pub fn get_user_email_from_repo(repo_path: &str) -> Result<String> {
    let repo_path = Path::new(repo_path);

    if !repo_path.exists() {
        bail!("Repository path does not exist: {}", repo_path.display());
    }

    if !repo_path.is_dir() {
        bail!("Repository path is not a directory: {}", repo_path.display());
    }

    let git_dir = repo_path.join(".git");
    if !git_dir.exists() {
        bail!("Not a git repository: {}", repo_path.display());
    }

    let output = Command::new("git")
        .args(["-C", repo_path.to_str().unwrap(), "config", "--local", "user.email"])
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        bail!("No local user.email configured in repository: {}", repo_path.display());
    }

    let email = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    if email.is_empty() {
        bail!("No local user.email configured in repository: {}", repo_path.display());
    }

    Ok(email)
}

/// Get both user.name and user.email from a specific repository
pub fn get_config_from_repo(repo_path: &str) -> Result<(String, String)> {
    let name = get_user_name_from_repo(repo_path)?;
    let email = get_user_email_from_repo(repo_path)?;
    Ok((name, email))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repo() {
        // This test assumes we're running in a git repository
        // The result depends on the test environment
        let _result = is_git_repo();
        // Can't assert much here without knowing the test environment
    }

    #[test]
    fn test_get_local_config_requires_git_repo() {
        // If not in a git repo, should return an error
        // This test is environment-dependent
        let result = get_local_config();
        // In a non-git directory, this should fail
        // In a git directory without local config, this should also fail
        if result.is_ok() {
            // If it succeeded, we must be in a git repo with local config
            let (name, email) = result.unwrap();
            assert!(!name.is_empty());
            assert!(!email.is_empty());
        }
    }

    #[test]
    fn test_get_global_config() {
        // Test getting global config
        // This is environment-dependent - global config may or may not be set
        let result = get_global_config();

        if result.is_ok() {
            // If it succeeded, both name and email should be non-empty
            let (name, email) = result.unwrap();
            assert!(!name.is_empty(), "Global user.name should not be empty");
            assert!(!email.is_empty(), "Global user.email should not be empty");
        }
        // If it failed, that's also valid (no global config set)
    }

    #[test]
    fn test_get_config_from_repo_with_invalid_path() {
        // Test with a non-existent path
        let result = get_config_from_repo("/nonexistent/path/to/repo");
        assert!(result.is_err(), "Should fail with non-existent path");
    }

    #[test]
    fn test_get_config_from_repo_with_non_git_dir() {
        // Test with a directory that exists but is not a git repo
        let result = get_config_from_repo("/tmp");
        assert!(result.is_err(), "Should fail with non-git directory");
    }

    #[test]
    fn test_get_config_from_repo_with_current_dir() {
        // Test with current directory if it's a git repo
        if is_git_repo() {
            // Try to get config from current directory
            let result = get_config_from_repo(".");

            // This might fail if local config is not set, which is valid
            if result.is_ok() {
                let (name, email) = result.unwrap();
                assert!(!name.is_empty());
                assert!(!email.is_empty());
            }
        }
    }

    #[test]
    fn test_global_user_name_consistency() {
        // If we can get global user name, getting it twice should return the same value
        if let Ok(name1) = get_global_user_name() {
            let name2 = get_global_user_name().unwrap();
            assert_eq!(name1, name2, "Global user.name should be consistent");
        }
    }

    #[test]
    fn test_global_user_email_consistency() {
        // If we can get global user email, getting it twice should return the same value
        if let Ok(email1) = get_global_user_email() {
            let email2 = get_global_user_email().unwrap();
            assert_eq!(email1, email2, "Global user.email should be consistent");
        }
    }
}
