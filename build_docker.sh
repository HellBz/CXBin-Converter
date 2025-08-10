#!/bin/bash
set -e

echo "🐳 Building Docker image..."
docker build -t cxbin-builder .

echo "📁 Creating bin directory..."
mkdir -p bin

echo "🚀 Running container to produce binary..."
docker run --rm -v "$(pwd)/bin:/out" cxbin-builder

echo "✅ Build complete. Binary is in ./bin"
