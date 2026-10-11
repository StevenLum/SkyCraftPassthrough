param([switch]$Offline, [switch]$PackageOnly)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$properties = Get-Content -Raw -LiteralPath (Join-Path $projectRoot 'minecraft\gradle.properties')
if ($properties -notmatch '(?m)^version=([0-9]+\.[0-9]+\.[0-9]+)\s*$') { throw 'Missing Minecraft package version.' }
$version = $Matches[1]
$manifest = Get-Content -Raw -LiteralPath (Join-Path $projectRoot 'passthrough\Cargo.toml')
if ($manifest -notmatch '(?m)^version = "([0-9]+\.[0-9]+\.[0-9]+)"\s*$' -or $Matches[1] -ne $version) { throw 'Rust and Minecraft versions must match.' }
# The Rust tests and the Minecraft-side rule checks must pass before anything is built or
# packaged; a failure stops here.
& (Join-Path $PSScriptRoot 'cargo.ps1') test --offline
& (Join-Path $PSScriptRoot 'test-java-rules.ps1')
& (Join-Path $PSScriptRoot 'cargo.ps1') build --release --offline
& (Join-Path $PSScriptRoot 'build-minecraft.ps1') -Offline:$Offline
$package = Join-Path $projectRoot 'dist\step5b'
$minecraft = Join-Path $package 'minecraft\mods'
$skyrim = Join-Path $package 'skyrim\SKSE\Plugins'
New-Item -ItemType Directory -Force $minecraft,$skyrim | Out-Null
$dll = Join-Path $projectRoot 'passthrough\target\x86_64-pc-windows-msvc\release\passthrough_link.dll'
$jar = Join-Path $projectRoot "minecraft\build\libs\passthrough-fabric-$version.jar"
if (-not (Test-Path $dll) -or -not (Test-Path $jar)) { throw 'Expected build artifacts are missing.' }
Copy-Item -LiteralPath $dll -Destination $minecraft -Force
Copy-Item -LiteralPath $dll -Destination $skyrim -Force
Copy-Item -LiteralPath $jar -Destination $minecraft -Force
Get-ChildItem -LiteralPath $minecraft -Filter 'passthrough-fabric-*.jar' -File |
    Where-Object Name -NE (Split-Path -Leaf $jar) | ForEach-Object { Remove-Item -LiteralPath $_.FullName -Force }
Compress-Archive -Path (Join-Path $package 'skyrim\SKSE') -DestinationPath (Join-Path $package "Passthrough-Skyrim-$version.zip") -Force
Compress-Archive -Path (Join-Path $package 'minecraft\mods') -DestinationPath (Join-Path $package "Passthrough-Minecraft-$version.zip") -Force
Write-Host "Built step 5b packages in $package. In-game behavior is not tested by this command."
if (-not $PackageOnly) { & (Join-Path $PSScriptRoot 'install-build.ps1') -PackageDirectory $package }
