use crate::config::WorkspaceConfig;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Default,
    Json,
    Table,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "default" | "text" => Some(Self::Default),
            "json" => Some(Self::Json),
            "table" => Some(Self::Table),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WorkspaceOutput {
    pub name: String,
    pub user_name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub patterns: Vec<String>,
}

impl From<(&String, &WorkspaceConfig)> for WorkspaceOutput {
    fn from((name, config): (&String, &WorkspaceConfig)) -> Self {
        Self {
            name: name.clone(),
            user_name: config.name.clone(),
            email: config.email.clone(),
            patterns: config.patterns.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StatusOutput {
    pub current_name: Option<String>,
    pub current_email: Option<String>,
    pub matching_workspace: Option<String>,
    pub available_workspaces: Vec<String>,
}

pub fn print_workspaces(workspaces: &HashMap<String, WorkspaceConfig>, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            let output: Vec<WorkspaceOutput> = workspaces
                .iter()
                .map(|(name, config)| (name, config).into())
                .collect();
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        OutputFormat::Table => {
            print_table(workspaces);
        }
        OutputFormat::Default => {
            print_default_workspaces(workspaces);
        }
    }
}

fn print_table(workspaces: &HashMap<String, WorkspaceConfig>) {
    if workspaces.is_empty() {
        println!("No workspaces configured.");
        return;
    }

    // Calculate column widths
    let mut workspaces_vec: Vec<_> = workspaces.iter().collect();
    workspaces_vec.sort_by_key(|(name, _)| *name);

    let max_workspace = workspaces_vec
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(9)
        .max(9);
    let max_name = workspaces_vec
        .iter()
        .map(|(_, config)| config.name.len())
        .max()
        .unwrap_or(4)
        .max(4);
    let max_email = workspaces_vec
        .iter()
        .map(|(_, config)| config.email.len())
        .max()
        .unwrap_or(5)
        .max(5);

    // Print header
    println!(
        "┌─{}─┬─{}─┬─{}─┐",
        "─".repeat(max_workspace),
        "─".repeat(max_name),
        "─".repeat(max_email)
    );
    println!(
        "│ {:<width_ws$} │ {:<width_name$} │ {:<width_email$} │",
        "Workspace",
        "Name",
        "Email",
        width_ws = max_workspace,
        width_name = max_name,
        width_email = max_email
    );
    println!(
        "├─{}─┼─{}─┼─{}─┤",
        "─".repeat(max_workspace),
        "─".repeat(max_name),
        "─".repeat(max_email)
    );

    // Print rows
    for (i, (workspace_name, config)) in workspaces_vec.iter().enumerate() {
        println!(
            "│ {:<width_ws$} │ {:<width_name$} │ {:<width_email$} │",
            workspace_name,
            config.name,
            config.email,
            width_ws = max_workspace,
            width_name = max_name,
            width_email = max_email
        );

        if i < workspaces_vec.len() - 1 {
            println!(
                "├─{}─┼─{}─┼─{}─┤",
                "─".repeat(max_workspace),
                "─".repeat(max_name),
                "─".repeat(max_email)
            );
        }
    }

    // Print footer
    println!(
        "└─{}─┴─{}─┴─{}─┘",
        "─".repeat(max_workspace),
        "─".repeat(max_name),
        "─".repeat(max_email)
    );
}

fn print_default_workspaces(workspaces: &HashMap<String, WorkspaceConfig>) {
    if workspaces.is_empty() {
        println!("No workspaces configured.");
        println!("Use 'figgit new <workspace> --name <name> --email <email>' to create one.");
        return;
    }

    println!("Configured workspaces:");
    println!();

    let mut workspaces_vec: Vec<_> = workspaces.iter().collect();
    workspaces_vec.sort_by_key(|(name, _)| *name);

    for (name, workspace_config) in workspaces_vec {
        println!("  {}:", name);
        println!("    Name:  {}", workspace_config.name);
        println!("    Email: {}", workspace_config.email);
        if !workspace_config.patterns.is_empty() {
            println!("    Patterns:");
            for pattern in &workspace_config.patterns {
                println!("      - {}", pattern);
            }
        }
        println!();
    }
}

pub fn print_status(status: &StatusOutput, format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(status).unwrap());
        }
        OutputFormat::Table | OutputFormat::Default => {
            print_default_status(status);
        }
    }
}

fn print_default_status(status: &StatusOutput) {
    println!("Git Configuration Status");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    if let (Some(name), Some(email)) = (&status.current_name, &status.current_email) {
        println!("Current local git config:");
        println!("  Name:  {}", name);
        println!("  Email: {}", email);
        println!();

        if let Some(workspace) = &status.matching_workspace {
            println!("✓ Matches workspace: '{}'", workspace);
        } else {
            println!("⚠ Does not match any configured workspace");

            if !status.available_workspaces.is_empty() {
                println!();
                println!("Available workspaces:");
                for workspace in &status.available_workspaces {
                    println!("  - {}", workspace);
                }
            }
        }
    } else {
        println!("⚠ Unable to read local git config");
        println!();

        if !status.available_workspaces.is_empty() {
            println!("Available workspaces:");
            for workspace in &status.available_workspaces {
                println!("  - {}", workspace);
            }
            println!();
            println!("Use 'figgit use <workspace>' to apply a workspace configuration.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("JSON"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("table"), Some(OutputFormat::Table));
        assert_eq!(
            OutputFormat::from_str("default"),
            Some(OutputFormat::Default)
        );
        assert_eq!(OutputFormat::from_str("text"), Some(OutputFormat::Default));
        assert_eq!(OutputFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_workspace_output_from_workspace_config() {
        let mut workspaces = HashMap::new();
        let config = WorkspaceConfig {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            patterns: Vec::new(),
        };
        let workspace_name = "test".to_string();
        workspaces.insert(workspace_name.clone(), config);

        let output: WorkspaceOutput =
            (&workspace_name, workspaces.get(&workspace_name).unwrap()).into();
        assert_eq!(output.name, "test");
        assert_eq!(output.user_name, "Test User");
        assert_eq!(output.email, "test@example.com");
    }
}
