#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${RED}Error: GitHub CLI (gh) is not installed${NC}"
    echo "Install it from: https://cli.github.com/"
    exit 1
fi

# Check if logged in to GitHub
if ! gh auth status &> /dev/null; then
    echo -e "${RED}Error: Not logged in to GitHub${NC}"
    echo "Run: gh auth login"
    exit 1
fi

# Bump version (default: patch)
BUMP_TYPE="${1:-patch}"
echo -e "${GREEN}Bumping ${BUMP_TYPE} version...${NC}"
AUTO_CONFIRM=1 ./scripts/bump-version.sh "$BUMP_TYPE" || exit 1

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
TAG="v${VERSION}"

echo -e "${GREEN}Creating release for version ${VERSION}${NC}"

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${YELLOW}Warning: Tag ${TAG} already exists${NC}"
    read -p "Do you want to delete and recreate it? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git tag -d "$TAG"
        git push origin ":refs/tags/$TAG" 2>/dev/null || true
    else
        echo "Aborted"
        exit 1
    fi
fi

# Build release binary
echo -e "${GREEN}Building release binary...${NC}"
cargo build --release

# Get binary path
BINARY="target/release/disk-cleanup-tool"

if [ ! -f "$BINARY" ]; then
    echo -e "${RED}Error: Binary not found at ${BINARY}${NC}"
    exit 1
fi

# Get binary size
SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
echo -e "${GREEN}Binary size: ${SIZE}${NC}"

# Create archive based on OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    darwin)
        OS_NAME="macos"
        ;;
    linux)
        OS_NAME="linux"
        ;;
    mingw*|msys*|cygwin*)
        OS_NAME="windows"
        BINARY="${BINARY}.exe"
        ;;
    *)
        OS_NAME="$OS"
        ;;
esac

ARCHIVE_NAME="disk-cleanup-tool-${VERSION}-${OS_NAME}-${ARCH}.tar.gz"
echo -e "${GREEN}Creating archive: ${ARCHIVE_NAME}${NC}"

# Create archive
tar -czf "$ARCHIVE_NAME" -C target/release disk-cleanup-tool
echo -e "${GREEN}Archive created: $(ls -lh "$ARCHIVE_NAME" | awk '{print $5}')${NC}"

# Prompt for release notes
echo -e "${YELLOW}Enter release notes (press Ctrl+D when done):${NC}"
RELEASE_NOTES=$(cat)

if [ -z "$RELEASE_NOTES" ]; then
    # Use changelog if no notes provided
    echo -e "${YELLOW}No release notes provided, extracting from CHANGELOG.md...${NC}"
    RELEASE_NOTES=$(sed -n "/## \[Unreleased\]/,/## \[/p" CHANGELOG.md | sed '1d;$d')
fi

# Create git tag
echo -e "${GREEN}Creating git tag ${TAG}...${NC}"
git tag -a "$TAG" -m "Release ${VERSION}"
git push origin "$TAG"

# Create GitHub release
echo -e "${GREEN}Creating GitHub release...${NC}"
gh release create "$TAG" \
    --title "Release ${VERSION}" \
    --notes "$RELEASE_NOTES" \
    "$ARCHIVE_NAME"

echo -e "${GREEN}✓ Release ${VERSION} created successfully!${NC}"
echo -e "${GREEN}✓ Archive uploaded: ${ARCHIVE_NAME}${NC}"
echo -e "${GREEN}View release: $(gh release view "$TAG" --web --json url -q .url)${NC}"

# Cleanup
rm "$ARCHIVE_NAME"
echo -e "${GREEN}✓ Cleaned up temporary files${NC}"
