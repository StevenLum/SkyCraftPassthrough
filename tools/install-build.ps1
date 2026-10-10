[CmdletBinding(SupportsShouldProcess=$true)]
param(
    [string]$PackageDirectory = (Join-Path (Split-Path -Parent $PSScriptRoot) 'dist\step5b')
)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$mcMods = 'C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\mods'
$skyPlugins = 'E:\SteamLibrary\steamapps\common\Skyrim Special Edition\Data\SKSE\Plugins'

# Java also hosts VS Code language servers and build tools. Match Minecraft,
# not the Java executable name. Never print command lines (they may hold tokens).
$running = @(Get-Process -Name SkyrimSE,skse64_loader -ErrorAction SilentlyContinue)
if ($running.Count) { throw "Close Skyrim before installing. Running process IDs: $($running.Id -join ', ')" }
try {
    $javaProcesses = @(Get-CimInstance Win32_Process -Filter "Name='java.exe' OR Name='javaw.exe'" -ErrorAction Stop)
    $minecraftProcesses = @($javaProcesses | Where-Object {
        $_.CommandLine -match 'net\.minecraft\.client\.main\.Main|net\.fabricmc\.loader\S*\.KnotClient|net\.minecraft\.launchwrapper\.Launch|org\.prismlauncher\.EntryPoint|org\.multimc\.EntryPoint' -or
        ($_.CommandLine -and $_.CommandLine.IndexOf((Split-Path -Parent $mcMods),[StringComparison]::OrdinalIgnoreCase) -ge 0)
    })
} catch {
    $minecraftProcesses = @()
    Write-Warning 'Java process details are unavailable; destination file access will still be checked. Keep both games closed.'
}
if ($minecraftProcesses.Count) { throw "Close Minecraft before installing. Running process IDs: $($minecraftProcesses.ProcessId -join ', ')" }
foreach ($directory in @($mcMods,$skyPlugins)) {
    if (-not (Test-Path -LiteralPath $directory -PathType Container)) { throw "Mod folder missing: $directory" }
}
$jars = @(Get-ChildItem -LiteralPath (Join-Path $PackageDirectory 'minecraft\mods') -Filter 'passthrough-fabric-*.jar' -File)
if ($jars.Count -ne 1) { throw 'Package must contain exactly one passthrough Fabric JAR. Rebuild or specify -PackageDirectory.' }
$files = @(
    [pscustomobject]@{ Source=$jars[0].FullName; Destination=(Join-Path $mcMods $jars[0].Name); Backup='minecraft' }
    [pscustomobject]@{ Source=(Join-Path $PackageDirectory 'minecraft\mods\passthrough_link.dll'); Destination=(Join-Path $mcMods 'passthrough_link.dll'); Backup='minecraft' }
    [pscustomobject]@{ Source=(Join-Path $PackageDirectory 'skyrim\SKSE\Plugins\passthrough_link.dll'); Destination=(Join-Path $skyPlugins 'passthrough_link.dll'); Backup='skyrim' }
)
foreach ($file in $files) {
    if (-not (Test-Path -LiteralPath $file.Source -PathType Leaf)) { throw "Package file missing: $($file.Source)" }
    $file | Add-Member -NotePropertyName Hash -NotePropertyValue (Get-FileHash -LiteralPath $file.Source -Algorithm SHA256).Hash
}
if ($files[1].Hash -ne $files[2].Hash) { throw 'Package DLLs do not match. Rebuild both adapters together.' }
$older = @(Get-ChildItem -LiteralPath $mcMods -Filter 'passthrough-fabric-*.jar' -File | Where-Object Name -NE $jars[0].Name)
if (-not $PSCmdlet.ShouldProcess("$mcMods and $skyPlugins", "Install $($jars[0].Name) and both DLLs; remove $($older.Count) older passthrough JAR(s)")) { return }

# Check all existing destinations before changing either game. Open does not
# truncate or write bytes. Windows rejects write access to a loaded plugin DLL.
$destinations = @($files | ForEach-Object { $_.Destination }) + @($older | ForEach-Object { $_.FullName })
foreach ($path in $destinations) {
    if (Test-Path -LiteralPath $path -PathType Leaf) {
        $handle = $null
        try { $handle = [IO.File]::Open($path,[IO.FileMode]::Open,[IO.FileAccess]::ReadWrite,[IO.FileShare]::None) }
        catch { throw "Cannot replace '$path'. Close the game or application using it and check write permissions. No installation files have been changed." }
        finally { if ($null -ne $handle) { $handle.Dispose() } }
    }
}

$backupRoot = Join-Path $projectRoot ('dist\install-backups\' + (Get-Date -Format 'yyyyMMdd-HHmmss-fff') + '-' + [guid]::NewGuid().ToString('N'))
$saved = @()
# Complete all backups before changing either installation.
foreach ($file in $files) {
    if (Test-Path -LiteralPath $file.Destination -PathType Leaf) {
        $directory = Join-Path $backupRoot $file.Backup
        New-Item -ItemType Directory -Force -Path $directory | Out-Null
        $backup = Join-Path $directory (Split-Path -Leaf $file.Destination)
        Copy-Item -LiteralPath $file.Destination -Destination $backup
        $saved += [pscustomobject]@{ Original=$file.Destination; Backup=$backup }
    }
}
foreach ($old in $older) {
    $directory = Join-Path $backupRoot 'minecraft'
    New-Item -ItemType Directory -Force -Path $directory | Out-Null
    $backup = Join-Path $directory $old.Name
    Copy-Item -LiteralPath $old.FullName -Destination $backup
    $saved += [pscustomobject]@{ Original=$old.FullName; Backup=$backup }
}
$attempted = @()
try {
    foreach ($file in $files) {
        $attempted += $file.Destination
        Copy-Item -LiteralPath $file.Source -Destination $file.Destination -Force
        if ((Get-FileHash -LiteralPath $file.Destination -Algorithm SHA256).Hash -ne $file.Hash) {
            throw "Installed checksum mismatch: $($file.Destination)"
        }
    }
    foreach ($old in $older) { Remove-Item -LiteralPath $old.FullName -Force }
} catch {
    $failure = $_
    try {
        foreach ($path in $attempted) {
            if ($path -notin $saved.Original -and (Test-Path -LiteralPath $path -PathType Leaf)) { Remove-Item -LiteralPath $path -Force }
        }
        foreach ($entry in $saved) { Copy-Item -LiteralPath $entry.Backup -Destination $entry.Original -Force }
        Write-Warning 'Install failed; restored the previous mod files.'
    } catch { Write-Warning "Restore also failed. Backups: $backupRoot. Error: $_" }
    throw $failure
}
foreach ($file in $files) { Write-Host "INSTALLED $($file.Destination) SHA256=$($file.Hash)" }
if ($saved.Count) { Write-Host "Previous files backed up to $backupRoot" }
Write-Host 'Install complete: 3 files copied and verified. You can start both games.'
