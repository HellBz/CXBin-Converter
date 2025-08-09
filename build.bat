@echo off
echo [🛠️] Checking Python & Pip...
python --version || (echo ❌ Python not found & pause & exit /b)
pip --version || (echo ❌ Pip not found & pause & exit /b)

echo [📦] Installing requirements...
pip install -r requirements.txt >nul 2>&1

echo [📦] Installing PyInstaller...
pip install pyinstaller >nul 2>&1

echo [🚧] Building cxbin_converter.exe...
pyinstaller --onefile --icon=icon.ico --name=cxbin_converter ^
  --hidden-import=networkx --hidden-import=lxml ^
  cxbin_converter/cxbin_converter.py

echo.
if exist dist\cxbin_converter.exe (
    echo ✅ Build successful: dist\cxbin_converter.exe
	
    if exist fallback.txt (
        copy /Y fallback.txt dist\ >nul
        echo 📄 Copied fallback.txt to dist\
    ) else (
        echo ⚠️ fallback.txt not found – skipping copy.
    )
) else (
    echo ❌ Build failed!
)

pause


