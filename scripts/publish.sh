#!/bin/bash

# Usage: ./scripts/publish.sh <version>

set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "❌ Please provide a version number"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

echo "🐕 Publishing Tazk v$VERSION"
echo "================================"

if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "❌ Invalid version format. Use semantic versioning (e.g., 1.0.0)"
    exit 1
fi

CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "❌ Please switch to main branch before publishing"
    echo "Current branch: $CURRENT_BRANCH"
    exit 1
fi

if ! git diff-index --quiet HEAD --; then
    echo "❌ You have uncommitted changes. Please commit or stash them first."
    exit 1
fi

echo "🧪 Running tests..."
cargo test

echo "📎 Running clippy..."
cargo clippy -- -D warnings

echo "✨ Checking formatting..."
cargo fmt --check

echo "✅ All checks passed!"
echo ""

echo "📝 Updating Cargo.toml version..."
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

echo "📝 Updating package.json version..."
sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json

echo "📝 Committing version changes..."
git add Cargo.toml Cargo.lock package.json
git commit -m "chore: bump version to $VERSION"

echo "🏷️  Creating git tag..."
git tag "v$VERSION"
git push origin main
git push origin "v$VERSION"

echo ""
echo "🚀 Version $VERSION has been tagged and pushed!"
echo ""
echo "Next steps:"
echo "1. 🏗️  GitHub Actions will automatically:"
echo "   - Build binaries for all platforms"
echo "   - Create a GitHub release"
echo "   - Publish to crates.io"
echo "   - Publish to npm"
echo ""
echo "3. 📊 Monitor the release:"
echo "   - GitHub Actions: https://github.com/nehu3n/tazk/actions"
echo "   - Crates.io: https://crates.io/crates/tazk"
echo "   - npm: https://www.npmjs.com/package/tazk-bin"
echo ""

echo "📋 Release Checklist:"
echo "   ✅ Tests passing"
echo "   ✅ Clippy checks passing"
echo "   ✅ Code formatted"
echo "   ✅ Version bumped in all files"
echo "   ✅ Git tag created and pushed"
echo "   ⏳ GitHub Actions building..."
echo "   ⏳ Publishing to crates.io..."
echo "   ⏳ Publishing to npm..."
echo ""
echo "🎉 Release initiated successfully!"
