@echo off
rem Starts Skyrim and Minecraft for the passthrough mod. All the work is in tools\play.ps1.
powershell -NoProfile -File "%~dp0tools\play.ps1" %*
rem On a problem the window stays until a key is pressed; otherwise it closes after a few seconds.
if errorlevel 1 (pause) else (ping -n 8 127.0.0.1 >nul)
