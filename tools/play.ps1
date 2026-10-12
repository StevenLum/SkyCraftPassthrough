param(
    # Print what would be started and start nothing.
    [switch]$Check,
    # Only needed if Prism Launcher is not found by itself.
    [string]$PrismLauncher
)
# Starts both games of the passthrough mod: Skyrim through SKSE and Minecraft through Prism
# Launcher. It checks first and changes nothing: no file is written in either game.
# Double-click Play.cmd in the project folder, or run .\tools\play.ps1.
$ErrorActionPreference = 'Stop'
$skyrimDirectory = 'E:\SteamLibrary\steamapps\common\Skyrim Special Edition'
$skyrimLoader = Join-Path $skyrimDirectory 'skse64_loader.exe'
$skyPlugin = Join-Path $skyrimDirectory 'Data\SKSE\Plugins\passthrough_link.dll'
$instance = '26.3'
$mcMods = "C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\$instance\.minecraft\mods"
$mcPlugin = Join-Path $mcMods 'passthrough_link.dll'

function Stop-With([string]$message) {
    Write-Host ''
    Write-Host "NOT STARTED: $message" -ForegroundColor Yellow
    exit 1
}

# Prism Launcher: the path given, else where Windows says it is installed.
if (-not $PrismLauncher) {
    $keys = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*', 'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
        'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*'
    $entry = Get-ItemProperty -Path $keys -ErrorAction SilentlyContinue |
        Where-Object { $_.DisplayName -eq 'Prism Launcher' -and $_.InstallLocation } | Select-Object -First 1
    if ($entry) { $PrismLauncher = Join-Path $entry.InstallLocation 'prismlauncher.exe' }
}
if (-not $PrismLauncher -or -not (Test-Path -LiteralPath $PrismLauncher -PathType Leaf)) {
    Stop-With 'Prism Launcher was not found. Run: .\tools\play.ps1 -PrismLauncher "<full path of prismlauncher.exe>"'
}
if (-not (Test-Path -LiteralPath $skyrimLoader -PathType Leaf)) { Stop-With "SKSE's loader is missing: $skyrimLoader" }

# The bridge file must be the same one in both games, or they refuse to connect.
foreach ($file in @($skyPlugin, $mcPlugin)) {
    if (-not (Test-Path -LiteralPath $file -PathType Leaf)) { Stop-With "The mod is not installed ($file is missing). Run .\tools\install-build.ps1 first." }
}
if ((Get-FileHash -LiteralPath $skyPlugin -Algorithm SHA256).Hash -ne (Get-FileHash -LiteralPath $mcPlugin -Algorithm SHA256).Hash) {
    Stop-With 'The two games have different versions of the mod. Close both and run .\tools\install-build.ps1.'
}
$jars = @(Get-ChildItem -LiteralPath $mcMods -Filter 'passthrough-fabric-*.jar' -File)
if ($jars.Count -ne 1) { Stop-With "Minecraft's mods folder has $($jars.Count) passthrough files instead of one. Run .\tools\install-build.ps1." }
$version = $jars[0].BaseName -replace '^passthrough-fabric-', ''
Write-Host "Passthrough mod $version is installed in both games."

# What is running already. Java also runs other programs, so Minecraft is recognised by its
# command line, as in install-build.ps1. Command lines are never printed.
$skyrimRunning = @(Get-Process -Name SkyrimSE, skse64_loader -ErrorAction SilentlyContinue).Count -gt 0
$minecraftRunning = $false
try {
    $java = @(Get-CimInstance Win32_Process -Filter "Name='java.exe' OR Name='javaw.exe'" -ErrorAction Stop)
    $minecraftRunning = @($java | Where-Object {
        $_.CommandLine -and $_.CommandLine.IndexOf((Split-Path -Parent $mcMods), [StringComparison]::OrdinalIgnoreCase) -ge 0
    }).Count -gt 0
} catch { Write-Host 'Could not tell whether Minecraft is already running; it will be started.' }

# Skyrim started without Steam starts Steam and then itself again, without SKSE and so
# without the mod.
if (-not $skyrimRunning -and @(Get-Process -Name steam -ErrorAction SilentlyContinue).Count -eq 0) {
    Stop-With 'Steam is not running. Start Steam, wait until it has signed in, then start this again.'
}

if ($skyrimRunning) { Write-Host 'Skyrim is already running; left as it is.' }
elseif ($Check) { Write-Host "Would start Skyrim: $skyrimLoader" }
else {
    Start-Process -FilePath $skyrimLoader -WorkingDirectory $skyrimDirectory
    Write-Host 'Started Skyrim (through SKSE).'
}
if ($minecraftRunning) { Write-Host 'Minecraft is already running; left as it is.' }
elseif ($Check) { Write-Host "Would start Minecraft: $PrismLauncher --launch $instance" }
else {
    Start-Process -FilePath $PrismLauncher -ArgumentList '--launch', $instance -WorkingDirectory (Split-Path -Parent $PrismLauncher)
    Write-Host "Started Minecraft (Prism Launcher, instance $instance)."
}
Write-Host ''
if ($Check) { Write-Host 'Check only: nothing was started.'; exit 0 }
Write-Host 'Still by hand:'
Write-Host '  1. In Skyrim: Continue, or load your paired save.'
Write-Host '  2. In Minecraft: Singleplayer, then your world.'
Write-Host '  3. Click into the Minecraft window to play.'
