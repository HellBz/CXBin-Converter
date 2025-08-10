#!/usr/bin/env bash
set -euo pipefail

# ---------------------------------------------
# Auto Installer + Runner for batch_all_formats.py
# - Finds Python (python3/python)
# - Creates/uses .venv if possible
# - Installs requirements.txt
# - Runs batch_all_formats.py on target dir
# Defaults:
#   target dir = .
#   zip mode   = --zip-only (can be overridden)
# ---------------------------------------------

# Resolve script directory (so relative paths work)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 1) Find Python
if command -v python3 >/dev/null 2>&1; then
  SYS_PY=python3
elif command -v python >/dev/null 2>&1; then
  SYS_PY=python
else
  echo "❌ Python not found in PATH."
  exit 1
fi

# 2) Setup venv if available (optional but recommended)
PY="$SYS_PY"
if "$SYS_PY" -c "import venv" >/dev/null 2>&1; then
  if [ ! -d ".venv" ]; then
    echo "🧪 Creating virtual environment (.venv)..."
    "$SYS_PY" -m venv .venv
  fi
  # shellcheck source=/dev/null
  source ".venv/bin/activate"
  PY=python
  echo "✅ Using virtual environment: .venv"
else
  echo "ℹ️  'venv' module not available, continuing without virtualenv."
fi

# 3) Install requirements (if file exists)
if [ -f "../requirements.txt" ]; then
  echo "📦 Installing dependencies from ../requirements.txt..."
  "$PY" -m pip install --upgrade pip >/dev/null
  "$PY" -m pip install -r ../requirements.txt
else
  echo "⚠️  requirements.txt not found – skipping dependency install."
fi

# 4) Prepare args
TARGET_DIR="${1:-.}"
shift || true
EXTRA_ARGS=("$@")

# If neither --zip nor --zip-only provided, default to --zip-only
ZIP_FLAG_PRESENT="no"
for a in "${EXTRA_ARGS[@]:-}"; do
  if [ "$a" = "--zip" ] || [ "$a" = "--zip-only" ]; then
    ZIP_FLAG_PRESENT="yes"
    break
  fi
done
if [ "$ZIP_FLAG_PRESENT" = "no" ]; then
  EXTRA_ARGS+=("--zip-only")
fi

# 5) Run the converter tester
echo "🚀 Running: batch_all_formats.py '$TARGET_DIR' ${EXTRA_ARGS[*]}"
exec "$PY" batch_all_formats.py "$TARGET_DIR" "${EXTRA_ARGS[@]}"
