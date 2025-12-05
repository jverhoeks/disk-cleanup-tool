#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

echo -e "${GREEN}Current version: ${CURRENT_VERSION}${NC}"

# Parse version components
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

# Determine bump type
BUMP_TYPE="${1:-patch}"

case "$BUMP_TYPE" in
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    patch)
        PATCH=$((PATCH + 1))
        ;;
    *)
        echo -e "${RED}Error: Invalid bump type '${BUMP_TYPE}'${NC}"
        echo "Usage: $0 [major|minor|patch]"
        exit 1
        ;;
esac

NEW_VERSION="${MAJOR}.${MINOR}.${PATCH}"
echo -e "${GREEN}New version: ${NEW_VERSION}${NC}"

# Confirm with user (skip if AUTO_CONFIRM is set)
if [ -z "$AUTO_CONFIRM" ]; then
    read -p "Bump version from ${CURRENT_VERSION} to ${NEW_VERSION}? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted"
        exit 1
    fi
fi

# Update Cargo.toml
echo -e "${GREEN}Updating Cargo.toml...${NC}"
sed -i.bak "s/^version = \"${CURRENT_VERSION}\"/version = \"${NEW_VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock
echo -e "${GREEN}Updating Cargo.lock...${NC}"
cargo build --release > /dev/null 2>&1

# Commit changes
echo -e "${GREEN}Committing version bump...${NC}"
git add Cargo.toml Cargo.lock
git commit -m "Bump version to ${NEW_VERSION}"

echo -e "${GREEN}✓ Version bumped to ${NEW_VERSION}${NC}"
echo -e "${YELLOW}Run 'make release' to create the GitHub release${NC}"
