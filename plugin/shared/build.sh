#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "🔨 Building pixly-eagle-core..."
cargo build --release

echo "📦 Copying binary to bin/"
mkdir -p bin
cp target/release/pixly-eagle-core bin/
chmod +x bin/pixly-eagle-core

echo "✅ Built: plugin/shared/bin/pixly-eagle-core"
echo ""
echo "Test with:"
echo "  ./bin/pixly-eagle-core --dev --version"
