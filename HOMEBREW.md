# Homebrew Distribution Guide

This guide explains how to distribute figgit via Homebrew.

## Option 1: Homebrew Tap (Recommended)

A Homebrew tap is a custom repository of formulas. This is the recommended approach for distributing your package.

### Setup

1. **Create a tap repository** on GitHub named `homebrew-figgit` (or `homebrew-<your-tap-name>`)
   - Repository must be public
   - Repository name must start with `homebrew-`

2. **Copy the formula** from `homebrew/Formula/figgit.rb` to your tap repository

3. **Update the formula** with your actual GitHub username:
   - Replace `USERNAME` with your GitHub username
   - After creating a release, update the SHA256 hashes

### Getting SHA256 Hashes

After creating a release, calculate SHA256 hashes for each platform:

```bash
# Download the release artifacts
wget https://github.com/USERNAME/figgit/releases/download/v0.1.0/figgit-x86_64-apple-darwin.tar.gz
wget https://github.com/USERNAME/figgit/releases/download/v0.1.0/figgit-aarch64-apple-darwin.tar.gz
wget https://github.com/USERNAME/figgit/releases/download/v0.1.0/figgit-x86_64-unknown-linux-gnu.tar.gz

# Calculate SHA256
shasum -a 256 figgit-x86_64-apple-darwin.tar.gz
shasum -a 256 figgit-aarch64-apple-darwin.tar.gz
shasum -a 256 figgit-x86_64-unknown-linux-gnu.tar.gz
```

Update the formula with these hashes.

### User Installation

Once your tap is set up, users can install figgit with:

```bash
# Add your tap
brew tap USERNAME/figgit

# Install figgit
brew install figgit
```

Or in one command:

```bash
brew install USERNAME/figgit
```

## Option 2: Submit to Homebrew Core

To make figgit available via `brew install figgit` (without a tap), you can submit it to Homebrew's core repository.

### Requirements

- Notable project (significant user base or utility)
- Stable version (1.0.0 or higher recommended)
- Active maintenance
- No conflicts with existing formulas

### Process

1. Create a release (v1.0.0 or higher)
2. Test the formula locally
3. Submit a pull request to [homebrew/homebrew-core](https://github.com/Homebrew/homebrew-core)

See [Homebrew's documentation](https://docs.brew.sh/Adding-Software-to-Homebrew) for more details.

## Automating Formula Updates

You can automate formula updates using GitHub Actions. Create a workflow that:

1. Triggers on new releases
2. Calculates SHA256 hashes
3. Updates the formula in your tap repository
4. Creates a PR or commits directly

Example workflow snippet:

```yaml
- name: Update Homebrew formula
  run: |
    VERSION=${{ github.ref_name }}

    # Download and hash artifacts
    wget https://github.com/${{ github.repository }}/releases/download/${VERSION}/figgit-x86_64-apple-darwin.tar.gz
    MACOS_AMD64_SHA=$(shasum -a 256 figgit-x86_64-apple-darwin.tar.gz | awk '{print $1}')

    # Update formula file
    sed -i "s/version \".*\"/version \"${VERSION#v}\"/" Formula/figgit.rb
    sed -i "s/REPLACE_WITH_ACTUAL_SHA256/${MACOS_AMD64_SHA}/" Formula/figgit.rb
```

## Testing Your Formula

Before publishing, test the formula locally:

```bash
# Install from local formula
brew install --build-from-source homebrew/Formula/figgit.rb

# Test the installation
figgit --help

# Audit the formula
brew audit --strict figgit

# Test the formula
brew test figgit

# Uninstall
brew uninstall figgit
```

## Distribution Checklist

- [ ] Create a GitHub release with version tag (e.g., v0.1.0)
- [ ] Release workflow builds and uploads binaries
- [ ] Download release artifacts and calculate SHA256 hashes
- [ ] Update formula with correct SHA256 hashes
- [ ] Update formula with your GitHub username
- [ ] Create homebrew-figgit repository (for tap distribution)
- [ ] Push formula to tap repository
- [ ] Test installation from tap
- [ ] Update README.md with installation instructions

## Alternative: Cargo Install

Users can also install via Cargo:

```bash
cargo install figgit
```

This is simpler but requires users to have Rust installed.
