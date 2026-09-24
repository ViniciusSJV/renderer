@echo off
chcp 65001 >nul
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0PLAYME.ps1" %*
set "playme_exit=%ERRORLEVEL%"
echo.
pause
exit /b %playme_exit%
