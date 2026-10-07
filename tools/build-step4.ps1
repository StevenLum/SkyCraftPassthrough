param([switch]$Offline)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
& (Join-Path $PSScriptRoot 'cargo.ps1') build --release --offline
& (Join-Path $PSScriptRoot 'build-minecraft.ps1') -Offline:$Offline
$package = Join-Path $projectRoot 'dist\step4'
$minecraft = Join-Path $package 'minecraft\mods'
$skyrim = Join-Path $package 'skyrim\SKSE\Plugins'
New-Item -ItemType Directory -Force $minecraft,$skyrim | Out-Null
$dll = Join-Path $projectRoot 'passthrough\target\x86_64-pc-windows-msvc\release\passthrough_link.dll'
$jar = Join-Path $projectRoot 'minecraft\build\libs\passthrough-fabric-0.4.0.jar'
if (-not (Test-Path $dll) -or -not (Test-Path $jar)) { throw 'Expected build artifacts are missing.' }
Copy-Item -LiteralPath $dll -Destination $minecraft -Force
Copy-Item -LiteralPath $dll -Destination $skyrim -Force
Copy-Item -LiteralPath $jar -Destination $minecraft -Force
Compress-Archive -Path (Join-Path $package 'skyrim\SKSE') -DestinationPath (Join-Path $package 'Passthrough-Skyrim-0.4.0.zip') -Force
Compress-Archive -Path (Join-Path $package 'minecraft\mods') -DestinationPath (Join-Path $package 'Passthrough-Minecraft-0.4.0.zip') -Force
Write-Host "Built step 4 packages in $package. Installation and in-game behavior are not tested by this command."
