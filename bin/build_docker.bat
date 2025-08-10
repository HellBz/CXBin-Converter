@echo off
setlocal enabledelayedexpansion

REM Resolve script and repo paths
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%\..") do set "REPO_ROOT=%%~fI"

echo [🐳] Building Docker image...
docker build -f "%SCRIPT_DIR%Dockerfile" -t cxbin-builder "%REPO_ROOT%"
if errorlevel 1 (
    echo ❌ Docker build failed!
    pause
    exit /b 1
)

REM Cleanup old tmp in bin/
if exist "%SCRIPT_DIR%\tmp" rmdir /S /Q "%SCRIPT_DIR%\tmp"
mkdir "%SCRIPT_DIR%\tmp"

echo [🚀] Running container to build and export to bin\tmp ...
docker run --rm -v "%SCRIPT_DIR%\tmp:/out" cxbin-builder
if errorlevel 1 (
    echo ❌ Docker run failed!
    pause
    exit /b 1
)

REM Move artifact from bin\tmp to bin\
if exist "%SCRIPT_DIR%\tmp\cxbin_converter.exe" (
    move /Y "%SCRIPT_DIR%\tmp\cxbin_converter.exe" "%SCRIPT_DIR%" >nul
) else if exist "%SCRIPT_DIR%\tmp\cxbin_converter" (
    move /Y "%SCRIPT_DIR%\tmp\cxbin_converter" "%SCRIPT_DIR%" >nul
) else (
    echo ❌ No binary found in bin\tmp!
    rmdir /S /Q "%SCRIPT_DIR%\tmp"
    pause
    exit /b 1
)

REM Cleanup tmp
rmdir /S /Q "%SCRIPT_DIR%\tmp"

echo ✅ Build complete. Binary is now in bin\
pause
endlocal
