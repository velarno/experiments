# Contributing to figgit

Thank you for your interest in contributing to figgit! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/figgit.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A code editor (VS Code, IntelliJ IDEA, or your preferred editor)

### Building

```bash
cargo build
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

### Code Formatting

We use `rustfmt` for code formatting. Before submitting a PR, please format your code:

```bash
cargo fmt
```

### Linting

We use `clippy` for linting. Please ensure your code passes clippy checks:

```bash
cargo clippy -- -D warnings
```

## Code Style

- Follow Rust naming conventions
- Write clear, descriptive commit messages
- Add tests for new functionality
- Update documentation as needed
- Keep functions focused and concise
- Add comments for complex logic

## Testing

- All new features should include unit tests
- Ensure all existing tests pass
- Test on multiple platforms if possible (Linux, macOS, Windows)
- Test error cases, not just happy paths

## Pull Request Process

1. **Update documentation**: Ensure README.md and other docs reflect your changes
2. **Add tests**: Include tests for new functionality
3. **Format code**: Run `cargo fmt`
4. **Pass clippy**: Run `cargo clippy -- -D warnings`
5. **Pass all tests**: Run `cargo test`
6. **Write clear commits**: Use descriptive commit messages
7. **Update CHANGELOG**: Add your changes to CHANGELOG.md (if it exists)
8. **Create PR**: Submit a pull request with a clear description

### PR Description Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
How has this been tested?

## Checklist
- [ ] Code follows the style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] No new warnings
```

## Commit Message Guidelines

We follow conventional commit format:

```
type(scope): subject

body

footer
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(commands): add list command to show all workspaces

Add a new 'list' command that displays all configured workspaces
in a table format with color coding.

Closes #123
```

```
fix(git): handle missing git config gracefully

Previously, the application would panic if git config was not set.
Now it returns a user-friendly error message.
```

## Bug Reports

When filing a bug report, please include:

- **Description**: Clear description of the bug
- **Steps to Reproduce**: Step-by-step instructions
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Rust version, figgit version
- **Logs**: Any relevant error messages or logs

## Feature Requests

When proposing a new feature:

- **Use Case**: Explain why this feature would be useful
- **Proposed Solution**: Describe how you envision it working
- **Alternatives**: Any alternative solutions you've considered
- **Additional Context**: Any other relevant information

## Code Review

All submissions require review. We aim to:

- Respond to PRs within a few days
- Provide constructive feedback
- Suggest improvements when needed
- Merge when all requirements are met

## Questions?

If you have questions:

- Check existing issues and PRs
- Open a new issue with the "question" label
- Be respectful and patient

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Thank You!

Your contributions make figgit better for everyone. Thank you for taking the time to contribute!
