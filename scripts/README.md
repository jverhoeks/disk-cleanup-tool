# Release Scripts

Scripts for creating GitHub releases.

## Prerequisites

Install GitHub CLI:
```bash
# macOS
brew install gh

# Linux
# See: https://github.com/cli/cli/blob/trunk/docs/install_linux.md

# Windows
# See: https://github.com/cli/cli#windows
```

Login to GitHub:
```bash
gh auth login
```

## Usage

### Quick Release (Recommended)

Automatically builds, creates archive, and publishes to GitHub with auto-generated notes:

```bash
./scripts/quick-release.sh
```

This will:
1. Read version from `Cargo.toml`
2. Build release binary
3. Create platform-specific archive
4. Create GitHub release with tag `v{VERSION}`
5. Upload archive as release asset
6. Auto-generate release notes from commits

### Full Release (Advanced)

More control with custom release notes:

```bash
./scripts/release.sh
```

This will:
1. Check for `gh` CLI and authentication
2. Read version from `Cargo.toml`
3. Build release binary
4. Create platform-specific archive
5. Prompt for release notes (or extract from CHANGELOG.md)
6. Create git tag
7. Create GitHub release
8. Upload archive

**Features:**
- Validates existing tags
- Extracts notes from CHANGELOG.md if not provided
- Color-coded output
- Error handling
- Automatic cleanup

## Before Releasing

### 1. Run Pre-Release Check

```bash
./scripts/pre-release-check.sh
```

This validates:
- ✅ Version in Cargo.toml
- ✅ Git working directory is clean
- ✅ All tests pass
- ✅ Release build succeeds
- ✅ CHANGELOG.md is updated
- ✅ Git tag doesn't exist
- ✅ GitHub CLI is installed and authenticated

### 2. Update Version

Update version in `Cargo.toml`:
```toml
[package]
version = "0.2.0"
```

### 3. Update CHANGELOG.md

```markdown
## [Unreleased]

### Added
- New feature X

### Fixed
- Bug Y
```

### 4. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.0"
git push
```

### 5. Run Release Script

```bash
./scripts/quick-release.sh
```

## Archive Format

Archives are named: `disk-cleanup-tool-{VERSION}-{OS}-{ARCH}.tar.gz`

Examples:
- `disk-cleanup-tool-0.1.0-macos-arm64.tar.gz`
- `disk-cleanup-tool-0.1.0-linux-x86_64.tar.gz`
- `disk-cleanup-tool-0.1.0-windows-x86_64.tar.gz`

## Troubleshooting

**"gh: command not found"**
- Install GitHub CLI (see Prerequisites)

**"Not logged in to GitHub"**
- Run: `gh auth login`

**"Tag already exists"**
- Delete tag: `git tag -d v0.1.0 && git push origin :refs/tags/v0.1.0`
- Or use the full release script which handles this

**"Binary not found"**
- Ensure `cargo build --release` completes successfully
- Check that `target/release/disk-cleanup-tool` exists

## Manual Release

If you prefer manual control:

```bash
# Build
cargo build --release

# Create archive
tar -czf disk-cleanup-tool.tar.gz -C target/release disk-cleanup-tool

# Create release
gh release create v0.1.0 \
    --title "Release 0.1.0" \
    --notes "Release notes here" \
    disk-cleanup-tool.tar.gz
```

## CI/CD Integration

### GitHub Actions (Automated)

The repository includes a GitHub Actions workflow (`.github/workflows/release.yml`) that automatically:

1. Triggers on tag push (e.g., `v0.1.0`)
2. Builds binaries for multiple platforms:
   - Linux (x86_64)
   - macOS (x86_64 and ARM64)
   - Windows (x86_64)
3. Creates archives for each platform
4. Uploads all archives to the GitHub release
5. Auto-generates release notes

**To use:**
```bash
# Update version in Cargo.toml, commit, then:
git tag v0.1.0
git push origin v0.1.0
```

The workflow will automatically build and publish the release with binaries for all platforms.

### Manual Scripts

Use the manual scripts when you want to:
- Create a release from your local machine
- Test the release process
- Create a release without CI/CD
