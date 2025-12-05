#!/bin/bash
# Pre-release checklist script

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔍 Pre-Release Checklist"
echo "========================"
echo ""

ERRORS=0
WARNINGS=0

# Check 1: Version in Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -n "Version in Cargo.toml: "
if [ -n "$VERSION" ]; then
    echo -e "${GREEN}${VERSION}${NC}"
else
    echo -e "${RED}NOT FOUND${NC}"
    ((ERRORS++))
fi

# Check 2: Git status
echo -n "Git working directory: "
if [ -z "$(git status --porcelain)" ]; then
    echo -e "${GREEN}Clean${NC}"
else
    echo -e "${YELLOW}Uncommitted changes${NC}"
    ((WARNINGS++))
fi

# Check 3: Tests pass
echo -n "Running tests: "
if cargo test --quiet 2>/dev/null; then
    echo -e "${GREEN}All tests pass${NC}"
else
    echo -e "${RED}Tests failed${NC}"
    ((ERRORS++))
fi

# Check 4: Build succeeds
echo -n "Release build: "
if cargo build --release --quiet 2>/dev/null; then
    echo -e "${GREEN}Success${NC}"
else
    echo -e "${RED}Build failed${NC}"
    ((ERRORS++))
fi

# Check 5: CHANGELOG updated
echo -n "CHANGELOG.md: "
if grep -q "## \[Unreleased\]" CHANGELOG.md && [ $(sed -n '/## \[Unreleased\]/,/## \[/p' CHANGELOG.md | wc -l) -gt 3 ]; then
    echo -e "${GREEN}Has unreleased changes${NC}"
else
    echo -e "${YELLOW}No unreleased changes${NC}"
    ((WARNINGS++))
fi

# Check 6: Tag doesn't exist
TAG="v${VERSION}"
echo -n "Git tag ${TAG}: "
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo -e "${YELLOW}Already exists${NC}"
    ((WARNINGS++))
else
    echo -e "${GREEN}Available${NC}"
fi

# Check 7: GitHub CLI
echo -n "GitHub CLI (gh): "
if command -v gh &> /dev/null; then
    if gh auth status &> /dev/null; then
        echo -e "${GREEN}Installed and authenticated${NC}"
    else
        echo -e "${YELLOW}Installed but not authenticated${NC}"
        ((WARNINGS++))
    fi
else
    echo -e "${YELLOW}Not installed (optional)${NC}"
fi

# Check 8: README mentions version
echo -n "README.md: "
if [ -f README.md ]; then
    echo -e "${GREEN}Exists${NC}"
else
    echo -e "${YELLOW}Not found${NC}"
    ((WARNINGS++))
fi

echo ""
echo "========================"

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ All checks passed! Ready to release.${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. ./scripts/quick-release.sh"
    echo "  OR"
    echo "  2. git tag v${VERSION} && git push origin v${VERSION}"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  ${WARNINGS} warning(s). Review before releasing.${NC}"
    exit 0
else
    echo -e "${RED}❌ ${ERRORS} error(s) found. Fix before releasing.${NC}"
    exit 1
fi
