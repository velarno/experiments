mod config;
mod git;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "figgit")]
#[command(about = "Manage git configurations using workspace names", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new workspace configuration
    New {
        /// Name of the workspace
        workspace: String,
        /// Git user name
        #[arg(short, long)]
        name: String,
        /// Git user email
        #[arg(short, long)]
        email: String,
    },
    /// Update an existing workspace configuration
    Update {
        /// Name of the workspace
        workspace: String,
        /// Git user name (optional)
        #[arg(short, long)]
        name: Option<String>,
        /// Git user email (optional)
        #[arg(short, long)]
        email: Option<String>,
    },
    /// Apply a workspace configuration to the local git repository
    Use {
        /// Name of the workspace
        workspace: String,
    },
    /// View a workspace configuration
    View {
        /// Name of the workspace (optional, shows all if not provided)
        workspace: Option<String>,
    },
    /// Show the current git configuration and compare with workspaces
    Status,
    /// Delete a workspace configuration
    Delete {
        /// Name of the workspace
        workspace: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { workspace, name, email } => {
            commands::new_workspace(&workspace, &name, &email)?;
        }
        Commands::Update { workspace, name, email } => {
            commands::update_workspace(&workspace, name.as_deref(), email.as_deref())?;
        }
        Commands::Use { workspace } => {
            commands::use_workspace(&workspace)?;
        }
        Commands::View { workspace } => {
            commands::view_workspace(workspace.as_deref())?;
        }
        Commands::Status => {
            commands::status()?;
        }
        Commands::Delete { workspace } => {
            commands::delete_workspace(&workspace)?;
        }
    }

    Ok(())
}
