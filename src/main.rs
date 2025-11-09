mod commands;
mod config;
mod git;
mod output;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use output::OutputFormat;

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
        /// URL patterns for auto-detection (can be specified multiple times)
        #[arg(short, long = "pattern", value_name = "PATTERN")]
        patterns: Vec<String>,
        /// Reset patterns list instead of appending
        #[arg(long)]
        reset: bool,
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
        /// Output format (default, json, table)
        #[arg(long, value_name = "FORMAT")]
        format: Option<String>,
        /// Output as JSON (shorthand for --format=json)
        #[arg(short = 'j', long = "json", conflicts_with = "format")]
        json: bool,
        /// Output as table (shorthand for --format=table)
        #[arg(short = 't', long = "table", conflicts_with = "format")]
        table: bool,
    },
    /// List all workspace configurations
    List {
        /// Output format (default, json, table)
        #[arg(long, value_name = "FORMAT")]
        format: Option<String>,
        /// Output as JSON (shorthand for --format=json)
        #[arg(short = 'j', long = "json", conflicts_with = "format")]
        json: bool,
        /// Output as table (shorthand for --format=table)
        #[arg(short = 't', long = "table", conflicts_with = "format")]
        table: bool,
    },
    /// Show the current git configuration and compare with workspaces
    Status {
        /// Output format (default, json, table)
        #[arg(long, value_name = "FORMAT")]
        format: Option<String>,
        /// Output as JSON (shorthand for --format=json)
        #[arg(short = 'j', long = "json", conflicts_with = "format")]
        json: bool,
    },
    /// Delete a workspace configuration
    Delete {
        /// Name of the workspace
        workspace: String,
    },
    /// Import a workspace configuration from git config
    Import {
        /// Name of the workspace
        workspace: String,
        /// Import from global git config instead of local
        #[arg(short, long)]
        global: bool,
        /// Import from a specific repository path
        #[arg(short, long)]
        from: Option<String>,
    },
    /// Generate shell completions
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn determine_format(format: Option<&str>, json: bool, table: bool) -> OutputFormat {
    if json {
        OutputFormat::Json
    } else if table {
        OutputFormat::Table
    } else if let Some(fmt) = format {
        OutputFormat::from_str(fmt).unwrap_or(OutputFormat::Default)
    } else {
        OutputFormat::Default
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            workspace,
            name,
            email,
        } => {
            commands::new_workspace(&workspace, &name, &email)?;
        }
        Commands::Update {
            workspace,
            name,
            email,
            patterns,
            reset,
        } => {
            commands::update_workspace(
                &workspace,
                name.as_deref(),
                email.as_deref(),
                patterns,
                reset,
            )?;
        }
        Commands::Use { workspace } => {
            commands::use_workspace(&workspace)?;
        }
        Commands::View {
            workspace,
            format,
            json,
            table,
        } => {
            let output_format = determine_format(format.as_deref(), json, table);
            commands::view_workspace(workspace.as_deref(), output_format)?;
        }
        Commands::List {
            format,
            json,
            table,
        } => {
            let output_format = determine_format(format.as_deref(), json, table);
            commands::list_workspaces(output_format)?;
        }
        Commands::Status { format, json } => {
            let output_format = determine_format(format.as_deref(), json, false);
            commands::status(output_format)?;
        }
        Commands::Delete { workspace } => {
            commands::delete_workspace(&workspace)?;
        }
        Commands::Import {
            workspace,
            global,
            from,
        } => {
            commands::import_workspace(&workspace, global, from.as_deref())?;
        }
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "figgit", &mut std::io::stdout());
        }
    }

    Ok(())
}
