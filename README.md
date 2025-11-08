# figgit

A command-line tool for managing git configurations using workspace names.

## Overview

`figgit` (config-git) helps you manage multiple git user configurations and easily switch between them. Perfect for developers who work on multiple projects with different git identities (work, personal, open-source, etc.).

## Features

- Store multiple git configurations as named workspaces
- Quickly switch between different git identities
- View current git configuration and compare with saved workspaces
- Simple TOML-based configuration storage
- Comprehensive error handling and user-friendly messages

## Installation

### From Source

```bash
cargo build --release
```

The binary will be available at `./target/release/figgit`.

To install globally:

```bash
cargo install --path .
```

## Usage

### Create a new workspace

```bash
figgit new <workspace> --name "<name>" --email "<email>"
```

Example:
```bash
figgit new work --name "John Doe" --email "john.doe@company.com"
figgit new personal --name "John Doe" --email "john@personal.com"
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

### Apply a workspace configuration

Apply a workspace to your current git repository:
```bash
figgit use work
```

This sets the local git `user.name` and `user.email` for the current repository.

### Update a workspace

Update the name:
```bash
figgit update work --name "Jane Doe"
```

Update the email:
```bash
figgit update work --email "jane.doe@company.com"
```

Update both:
```bash
figgit update work --name "Jane Doe" --email "jane.doe@company.com"
```

### Check current status

See your current git configuration and which workspace it matches:
```bash
figgit status
```

### Delete a workspace

```bash
figgit delete work
```

## Configuration

Workspaces are stored in `~/.config/figgit/config.toml`. The configuration file is created automatically when you create your first workspace.

Example configuration:
```toml
[workspaces.work]
name = "John Doe"
email = "john.doe@company.com"

[workspaces.personal]
name = "John Doe"
email = "john@personal.com"
```

## Architecture

The project is organized into several modules:

- `main.rs` - CLI entry point and command routing
- `config.rs` - Configuration management and TOML storage
- `git.rs` - Git operations (reading/writing local config)
- `commands.rs` - Implementation of all subcommands

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
figgit new work --name "Jane Developer" --email "jane@company.com"
figgit new personal --name "Jane" --email "jane@personal.email"

# In a work repository
cd ~/work/project
figgit use work
figgit status  # Shows: ✓ Matches workspace: 'work'

# In a personal repository
cd ~/personal/project
figgit use personal
figgit status  # Shows: ✓ Matches workspace: 'personal'
```

## Requirements

- Rust 1.70 or later
- Git installed and available in PATH

## License

This project is available under the MIT license.

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting a pull request.
