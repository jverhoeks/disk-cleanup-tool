#!/bin/bash
# Quick release script - minimal version
set -e

# Bump version (default: patch)
BUMP_TYPE="${1:-patch}"
echo "🔢 Bumping ${BUMP_TYPE} version..."
AUTO_CONFIRM=1 ./scripts/bump-version.sh "$BUMP_TYPE" || exit 1

VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
TAG="v${VERSION}"

echo "🚀 Creating release ${TAG}"

# Build
echo "📦 Building..."
cargo build --release

# Create archive
ARCHIVE="disk-cleanup-tool-${VERSION}-$(uname -s)-$(uname -m).tar.gz"
tar -czf "$ARCHIVE" -C target/release disk-cleanup-tool

# Create release
echo "🎉 Creating GitHub release..."
gh release create "$TAG" \
    --title "Release ${VERSION}" \
    --generate-notes \
    "$ARCHIVE"

# Cleanup
rm "$ARCHIVE"

echo "✅ Done! View at: https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/releases/tag/${TAG}"
