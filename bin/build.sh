#!/usr/bin/env bash
set -euo pipefail

echo "[🛠️] Checking Python & Pip..."
if ! python3 --version >/dev/null 2>&1; then
    echo "❌ Python not found"
    exit 1
fi
if ! pip3 --version >/dev/null 2>&1; then
    echo "❌ Pip not found"
    exit 1
fi

echo "[📦] Installing requirements..."
pip3 install -r requirements.txt >/dev/null 2>&1

e
echo "[📦] Removing old PyInstaller..."
pip3 uninstall -y pyinstaller >/dev/null 2>&1 || true
pip3 cache purge >/dev/null 2>&1 || true

echo "[📦] Installing build tools..."
sudo apt-get update -y >/dev/null 2>&1
sudo apt-get install -y build-essential python3-dev wget >/dev/null 2>&1

echo "[⬇️] Downloading PyInstaller 6.15.0 source..."
TMPDIR=$(mktemp -d)
cd "$TMPDIR"
wget -q https://files.pythonhosted.org/packages/source/p/pyinstaller/pyinstaller-6.15.0.tar.gz

echo "[📦] Extracting PyInstaller..."
tar xvf pyinstaller-6.15.0.tar.gz >/dev/null 2>&1
cd pyinstaller-6.15.0

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Define PyInstaller args
ENTRY="$REPO_ROOT/cxbin_converter/cxbin_converter.py"
ICON="$SCRIPT_DIR/icon.ico"
DIST="$SCRIPT_DIR"
WORK="$SCRIPT_DIR/build"
SPEC="$SCRIPT_DIR"

echo "[🚧] Building cxbin_converter..."
pyinstaller "$ENTRY" \
    --name cxbin_converter \
    --onefile \
    --icon "$ICON" \
    --distpath "$DIST" \
    --workpath "$WORK" \
    --specpath "$SPEC" \
    --clean \
    --log-level=DEBUG

echo
if [ -f "$DIST/cxbin_converter" ]; then
    echo "✅ Build successful: $DIST/cxbin_converter"

    # Cleanup: remove .spec file and build folder
    if [ -f "$SPEC/cxbin_converter.spec" ]; then
        rm -f "$SPEC/cxbin_converter.spec"
    fi
    if [ -d "$WORK" ]; then
        rm -rf "$WORK"
    fi
    echo "🧹 Cleanup done: Removed build folder and .spec file."
else
    echo "❌ Build failed!"
    exit 1
fi
