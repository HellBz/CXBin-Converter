#!/usr/bin/env bash
set -euo pipefail

# URL zur libpython3.11.a aus deinem Release Asset
LIBPYTHON_URL="https://github.com/HellBz/CXBin-Converter/releases/download/1/libpython3.11.a"

echo "[🛠️] Checking Python & Pip..."
if ! python3 --version >/dev/null 2>&1; then
    echo "❌ Python not found"
    exit 1
fi
if ! pip3 --version >/dev/null 2>&1; then
    echo "❌ Pip not found"
    exit 1
fi

echo "[⬇️] Downloading libpython3.11.a..."
curl -L -o libpython3.11.a "$LIBPYTHON_URL"
echo "✅ libpython3.11.a downloaded."

echo "[📦] Installing requirements..."
pip3 install -r requirements.txt >/dev/null 2>&1

echo "[📦] Installing PyInstaller..."
pip3 install pyinstaller >/dev/null 2>&1

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
    --static-libpython \
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
