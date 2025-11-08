use anyhow::{Context, Result, bail};
use std::process::Command;
use std::path::Path;

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
        .args(&["config", "--local", "user.name"])
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
        .args(&["config", "--local", "user.email"])
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
        .args(&["config", "--local", "user.name", name])
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
        .args(&["config", "--local", "user.email", email])
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
}
