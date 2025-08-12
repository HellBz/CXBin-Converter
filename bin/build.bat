@echo off
setlocal

echo [🛠️] Checking Python & Pip...
python --version >nul 2>&1 || (echo ❌ Python not found & pause & exit /b)
pip --version >nul 2>&1 || (echo ❌ Pip not found & pause & exit /b)

echo [📦] Installing requirements...
pip install -r requirements.txt >nul 2>&1

echo [📦] Installing PyInstaller...
pip install pyinstaller >nul 2>&1

REM Resolve paths
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%\..") do set "REPO_ROOT=%%~fI"

REM Define PyInstaller args (use backslashes, no trailing slashes)
set "ENTRY=%REPO_ROOT%\cxbin_converter\cxbin_converter.py"
set "ICON=%SCRIPT_DIR%\icon.ico"
set "DIST=%SCRIPT_DIR%\"
set "WORK=%SCRIPT_DIR%\build"
set "SPEC=%SCRIPT_DIR%\"

REM Build directly into bin (EXE in bin, temp in bin\build)
echo [🚧] Building cxbin_converter.exe...
pyinstaller "%ENTRY%" --name cxbin_converter --onefile --icon "%ICON%" --distpath "%DIST%" --workpath "%WORK%" --specpath "%SPEC%" --clean --log-level=DEBUG

echo.
if exist "%DIST%\cxbin_converter.exe" (
    echo ✅ Build successful: %DIST%\cxbin_converter.exe

    REM Cleanup: remove .spec file and build folder
    if exist "%SPEC%\cxbin_converter.spec" del /F /Q "%SPEC%\cxbin_converter.spec"
    if exist "%WORK%" rmdir /S /Q "%WORK%"
    echo 🧹 Cleanup done: Removed build folder and .spec file.
) else (
    echo ❌ Build failed!
    exit /b 1
)

pause
endlocal


