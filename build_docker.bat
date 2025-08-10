@echo off
REM Build Docker Image
docker build -t cxbin-builder .

REM bin-Ordner erstellen, falls nicht vorhanden
if not exist bin mkdir bin

REM Aktuelles Verzeichnis ermitteln
for /f "delims=" %%i in ('cd') do set CUR=%%i

REM Binary erzeugen und nach .\bin mappen
docker run --rm -v "%CUR%\bin:/out" cxbin-builder

echo.
echo ✅ Ergebnis in .\bin\
pause
