# Release Scripts

Scripts for creating GitHub releases with automatic version bumping.

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

Automatically bumps version, builds, creates archive, and publishes to GitHub with auto-generated notes:

```bash
# Bump patch version (0.1.0 -> 0.1.1)
make release

# Or specify bump type
make release BUMP=minor  # 0.1.0 -> 0.2.0
make release BUMP=major  # 0.1.0 -> 1.0.0
```

Or directly:
```bash
./scripts/quick-release.sh        # patch bump
./scripts/quick-release.sh minor  # minor bump
./scripts/quick-release.sh major  # major bump
```

This will:
1. Bump version in `Cargo.toml` (patch/minor/major)
2. Update `Cargo.lock`
3. Commit version bump
4. Build release binary
5. Create platform-specific archive
6. Create GitHub release with tag `v{VERSION}`
7. Upload archive as release asset
8. Auto-generate release notes from commits

### Full Release (Advanced)

More control with custom release notes:

```bash
./scripts/release.sh        # patch bump
./scripts/release.sh minor  # minor bump
./scripts/release.sh major  # major bump
```

This will:
1. Check for `gh` CLI and authentication
2. Bump version in `Cargo.toml` (patch/minor/major)
3. Update `Cargo.lock`
4. Commit version bump
5. Build release binary
6. Create platform-specific archive
7. Prompt for release notes (or extract from CHANGELOG.md)
8. Create git tag
9. Create GitHub release
10. Upload archive

**Features:**
- Validates existing tags
- Extracts notes from CHANGELOG.md if not provided
- Color-coded output
- Error handling
- Automatic cleanup

## Before Releasing

### 1. Update CHANGELOG.md (Optional)

```markdown
## [Unreleased]

### Added
- New feature X

### Fixed
- Bug Y
```

### 2. Run Release

The release script will automatically:
- Bump the version
- Update Cargo.toml and Cargo.lock
- Commit the version bump
- Create the release

```bash
make release              # patch: 0.1.0 -> 0.1.1
make release BUMP=minor   # minor: 0.1.0 -> 0.2.0
make release BUMP=major   # major: 0.1.0 -> 1.0.0
```

### Manual Version Bump (Optional)

If you want to bump the version without releasing:

```bash
./scripts/bump-version.sh        # patch bump
./scripts/bump-version.sh minor  # minor bump
./scripts/bump-version.sh major  # major bump
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
