#!/bin/bash

echo "🛠️ Checking Python..."

# Try python3 first, fallback to python
if command -v python3 &>/dev/null; then
    PYTHON=python3
elif command -v python &>/dev/null; then
    PYTHON=python
else
    echo "❌ Python not found. Please install Python."
    exit 1
fi

# Check for pip
if ! $PYTHON -m pip --version &>/dev/null; then
    echo "❌ pip not found. Please install pip (e.g. $PYTHON -m ensurepip)"
    exit 1
fi

echo "📦 Installing Python dependencies..."
$PYTHON -m pip install -r requirements.txt

echo "📦 Installing PyInstaller..."
$PYTHON -m pip install pyinstaller

echo "🚧 Building cxbin_converter..."
pyinstaller --onefile \
    --name=cxbin_converter \
	--version-file version.txt \
    --hidden-import=networkx \
    --hidden-import=lxml \
    cxbin_converter.py

echo ""

if [[ -f "dist/cxbin_converter" ]]; then
    echo "✅ Build successful: dist/cxbin_converter"
	
    if [[ -f "fallback.txt" ]]; then
        cp fallback.txt dist/
        echo "📄 Copied fallback.txt to dist/"
    else
        echo "⚠️ fallback.txt not found – skipping copy."
    fi
else
    echo "❌ Build failed."
fi
