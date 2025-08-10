#!/bin/bash
set -e

# Determine paths relative to this script
SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🐳 Building Docker image..."
docker build -f "$SCRIPT_DIR/Dockerfile" -t cxbin-builder "$REPO_ROOT"

# tmp inside bin/
rm -rf "$SCRIPT_DIR/tmp"
mkdir -p "$SCRIPT_DIR/tmp"

echo "🚀 Running container to build and export to bin/tmp ..."
docker run --rm -v "$SCRIPT_DIR/tmp:/out" cxbin-builder

# Move artifact from bin/tmp → bin/
if [ -f "$SCRIPT_DIR/tmp/cxbin_converter" ] || [ -f "$SCRIPT_DIR/tmp/cxbin_converter.exe" ]; then
  echo "📦 Moving built binary to bin/ ..."
  mv "$SCRIPT_DIR"/tmp/cxbin_converter* "$SCRIPT_DIR"/
else
  echo "❌ No binary found in bin/tmp !"
  rm -rf "$SCRIPT_DIR/tmp"
  exit 1
fi

# Cleanup tmp
rm -rf "$SCRIPT_DIR/tmp"
echo "✅ Build complete. Binary is now in bin/"
