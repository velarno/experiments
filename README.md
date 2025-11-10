# figgit

[![CI](https://github.com/velarno/figgit/actions/workflows/ci.yml/badge.svg)](https://github.com/velarno/figgit/actions/workflows/ci.yml)
[![Release](https://github.com/velarno/figgit/actions/workflows/release.yml/badge.svg)](https://github.com/velarno/figgit/actions/workflows/release.yml)

A command-line tool for managing git configurations using workspace names.

## Overview

`figgit` (config-git) helps you manage multiple git user configurations and easily switch between them. Perfect for developers who work on multiple projects with different git identities (work, personal, open-source, etc.).

## Features

- Store multiple git configurations as named workspaces
- Quickly switch between different git identities
- Import existing git configurations from local, global, or other repositories
- URL pattern matching for workspace auto-detection
- Multiple output formats: default text, formatted tables, and JSON
- View current git configuration and compare with saved workspaces
- Shell completion support for Bash, Zsh, Fish, and PowerShell
- Simple TOML-based configuration storage
- Comprehensive error handling and user-friendly messages
- Cross-platform support (Linux, macOS, Windows)

## Installation

### Homebrew (macOS and Linux)

```bash
# Add the tap
brew tap velarno/figgit

# Install figgit
brew install figgit
```

See [HOMEBREW.md](HOMEBREW.md) for detailed Homebrew distribution instructions.

### Cargo

```bash
cargo install figgit
```

### From Source

```bash
git clone https://github.com/velarno/figgit.git
cd figgit
cargo build --release
```

The binary will be available at `./target/release/figgit`.

### Pre-built Binaries

Download pre-built binaries for your platform from the [releases page](https://github.com/velarno/figgit/releases).

## Usage

### Create a new workspace

```bash
figgit new <workspace> --name "<name>" --email "<email>"
# Or using short flags:
figgit new <workspace> -n "<name>" -e "<email>"
```

Example:
```bash
figgit new work -n "John Doe" -e "john.doe@company.com"
figgit new personal -n "John Doe" -e "john@personal.com"
```

### View workspaces

View all configured workspaces:
```bash
figgit view
```

View a specific workspace:
```bash
figgit view work
```

### List workspaces

List all workspaces (alternative to `view`):
```bash
figgit list
```

With different output formats:
```bash
# Table format
figgit list -t
# Or: figgit list --format=table

# JSON format
figgit list -j
# Or: figgit list --format=json
```

### Output formats

Most display commands support different output formats:

- **Default**: Human-readable text format (default)
- **Table**: Formatted table with borders
- **JSON**: Machine-readable JSON format

Available on: `list`, `view`, `status`

Examples:
```bash
# View all workspaces as a table
figgit view -t

# View specific workspace as JSON
figgit view work -j

# Check status in JSON format
figgit status -j
```

### Apply a workspace configuration

Apply a workspace to your current git repository:
```bash
figgit use work
```

This sets the local git `user.name` and `user.email` for the current repository.

### Update a workspace

Update the name:
```bash
figgit update work -n "Jane Doe"
```

Update the email:
```bash
figgit update work -e "jane.doe@company.com"
```

Update both:
```bash
figgit update work -n "Jane Doe" -e "jane.doe@company.com"
```

Add URL patterns for auto-detection (append to existing):
```bash
figgit update work -p "github.com/company/*"
figgit update work -p "gitlab.company.com/*" -p "bitbucket.org/company/*"
```

Reset patterns (replace all existing patterns):
```bash
figgit update work -p "github.com/newcompany/*" --reset
```

URL patterns support glob-style wildcards and can be used for future auto-detection features.

### Import from existing git config

Import a workspace from your current repository's local git config:
```bash
figgit import work
```

Import from global git config:
```bash
figgit import personal --global
```

Import from another repository:
```bash
figgit import client-project --from /path/to/repo
```

This is especially useful when you want to save your existing git configurations as workspaces.

### Check current status

See your current git configuration and which workspace it matches:
```bash
figgit status
```

### Delete a workspace

```bash
figgit delete work
```

### Shell completions

Generate shell completions for your shell:

**Bash:**
```bash
figgit completion bash > ~/.local/share/bash-completion/completions/figgit
```

**Zsh:**
```bash
figgit completion zsh > ~/.zsh/completions/_figgit
# Add to ~/.zshrc: fpath=(~/.zsh/completions $fpath)
```

**Fish:**
```bash
figgit completion fish > ~/.config/fish/completions/figgit.fish
```

**PowerShell:**
```powershell
figgit completion powershell | Out-String | Invoke-Expression
```

## Configuration

Workspaces are stored in `~/.config/figgit/config.toml`. The configuration file is created automatically when you create your first workspace.

Example configuration:
```toml
[workspaces.work]
name = "John Doe"
email = "john.doe@company.com"
patterns = ["github.com/company/*", "gitlab.company.com/*"]

[workspaces.personal]
name = "John Doe"
email = "john@personal.com"
```

The `patterns` field is optional and can be used to associate URL patterns with workspaces for future auto-detection features.

## Architecture

The project is organized into several modules:

- `main.rs` - CLI entry point, command routing, and shell completion
- `config.rs` - Configuration management and TOML storage
- `git.rs` - Git operations (reading/writing local and global config)
- `commands.rs` - Implementation of all subcommands
- `output.rs` - Output formatting (default, table, JSON)

### Key Design Decisions

1. **Local-only git config**: The tool only modifies local git configuration (`.git/config`), never global config, to prevent accidental changes to your default git identity.

2. **TOML storage**: Configuration is stored in a simple, human-readable TOML format.

3. **Error handling**: Uses `anyhow` for ergonomic error handling with context.

4. **CLI framework**: Uses `clap` with derive macros for type-safe, maintainable CLI definitions.

5. **Testing**: Comprehensive unit tests for all core functionality.

## Testing

Run the test suite:
```bash
cargo test
```

All modules include unit tests that cover:
- Configuration CRUD operations
- Workspace matching and comparison
- Serialization/deserialization
- Error cases

## Examples

### Typical workflow for a developer with work and personal projects

```bash
# Set up your workspaces once
figgit new work -n "Jane Developer" -e "jane@company.com"
figgit new personal -n "Jane" -e "jane@personal.email"

# In a work repository
cd ~/work/project
figgit use work
figgit status  # Shows: ✓ Matches workspace: 'work'

# In a personal repository
cd ~/personal/project
figgit use personal
figgit status  # Shows: ✓ Matches workspace: 'personal'
```

### Importing existing configurations

If you already have git configured in various repositories, you can import them:

```bash
# In your work repository with existing config
cd ~/work/project
figgit import work
# Imports the local config from this repo as 'work' workspace

# Import your global config as a workspace
figgit import personal --global

# Import config from another repository
figgit import opensource --from ~/oss/project

# Now you can use these workspaces anywhere
cd ~/new/work/project
figgit use work
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

- **CI Workflow**: Tests on Linux, macOS, and Windows on every push and pull request
- **Release Workflow**: Builds binaries for all platforms when a version tag is pushed
- **Code Coverage**: Tracks test coverage using cargo-tarpaulin

To create a new release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow will automatically:
- Build binaries for all supported platforms
- Create a GitHub release
- Upload the binaries as release artifacts
- Publish to crates.io (if configured)

## Requirements

- Rust 1.70 or later
- Git installed and available in PATH

## License

This project is available under the MIT license.

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting a pull request.
