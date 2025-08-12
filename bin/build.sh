#!/usr/bin/env bash
set -euo pipefail

PYTHON_VERSION="3.11.9"
SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ENTRY="$REPO_ROOT/cxbin_converter/cxbin_converter.py"
ICON="$SCRIPT_DIR/icon.ico"
DIST="$SCRIPT_DIR"
WORK="$SCRIPT_DIR/build"
SPEC="$SCRIPT_DIR"

echo "[🛠️] Installing build dependencies..."
sudo apt-get update
sudo apt-get install -y build-essential wget libffi-dev zlib1g-dev libssl-dev \
    libbz2-dev libsqlite3-dev libreadline-dev libncursesw5-dev \
    libgdbm-dev liblzma-dev uuid-dev

echo "[⬇️] Downloading & building Python $PYTHON_VERSION (static)..."
cd /tmp
wget "https://www.python.org/ftp/python/$PYTHON_VERSION/Python-$PYTHON_VERSION.tgz"
tar xzf "Python-$PYTHON_VERSION.tgz"
cd "Python-$PYTHON_VERSION"
./configure --enable-optimizations --disable-shared
make -j$(nproc)
sudo make altinstall

echo "[📦] Installing PyInstaller..."
pip3.$(echo $PYTHON_VERSION | cut -d. -f1,2) install --upgrade pip
pip3.$(echo $PYTHON_VERSION | cut -d. -f1,2) install pyinstaller

echo "[🚧] Building cxbin_converter with static libpython..."
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
    rm -f "$SPEC/cxbin_converter.spec"
    rm -rf "$WORK"
    echo "🧹 Cleanup done."
else
    echo "❌ Build failed!"
    exit 1
fi
