@echo off
setlocal enabledelayedexpansion

echo [🛠️] Checking Python & Pip...
python --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Python not found
    pause
    exit /b
)

pip --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Pip not found
    pause
    exit /b
)

echo [📦] Installing requirements...
if exist "..\requirements.txt" (
    python -m pip install --upgrade pip >nul
    python -m pip install -r ..\requirements.txt
) else (
    echo ⚠️ ..\requirements.txt not found – skipping dependency install.
)

echo [🚧] Running batch_all_formats.py...
python batch_all_formats.py .\ --zip-only

echo.
echo ✅ Batch conversion done.
pause
