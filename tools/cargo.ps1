# Run Cargo with project-local caches and Windows x64 compiler libraries.
# Example: .\tools\cargo.ps1 build --offline
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$names = @('RUSTUP_HOME', 'CARGO_HOME', 'CARGO_TARGET_DIR', 'CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER', 'LIB', 'TEMP', 'TMP')
$previous = @{}
foreach ($name in $names) { $previous[$name] = [Environment]::GetEnvironmentVariable($name, 'Process') }
try {
    $localCargo = Join-Path $projectRoot '.tools\cargo\bin\cargo.exe'
    if (Test-Path -LiteralPath $localCargo) {
        $cargo = $localCargo
        $env:RUSTUP_HOME = Join-Path $projectRoot '.tools\rustup'
    } else {
        $command = Get-Command cargo.exe -ErrorAction SilentlyContinue
        if (-not $command) { throw 'Rust stable MSVC is required. See README.md.' }
        $cargo = $command.Source
    }
    $env:CARGO_HOME = Join-Path $projectRoot '.tools\cargo'
    $env:CARGO_TARGET_DIR = Join-Path $projectRoot 'passthrough\target'
    $env:TEMP = Join-Path $projectRoot '.tools\temp'
    $env:TMP = $env:TEMP
    New-Item -ItemType Directory -Force $env:TEMP | Out-Null

    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
    if (-not (Test-Path -LiteralPath $vswhere)) { throw 'Visual Studio C++ Build Tools are required. See README.md.' }
    $visualStudio = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if (-not $visualStudio) { throw 'No installed MSVC x64 compiler was found.' }
    $compiler = Get-ChildItem -Directory (Join-Path $visualStudio 'VC\Tools\MSVC') |
        Sort-Object { [version]$_.Name } -Descending | Select-Object -First 1
    $linker = Join-Path $compiler.FullName 'bin\Hostx64\x64\link.exe'
    if (-not (Test-Path -LiteralPath $linker)) { throw 'The MSVC x64 linker is missing.' }
    $env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = $linker
    $libraries = @((Join-Path $compiler.FullName 'lib\x64'))

    $portableSdk = Join-Path $projectRoot '.tools\windows-sdk-x64\c'
    if (Test-Path -LiteralPath (Join-Path $portableSdk 'um\x64\kernel32.lib')) {
        $sdk = $portableSdk
    } else {
        $sdkRoot = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\Lib'
        $sdk = Get-ChildItem -Directory $sdkRoot -ErrorAction SilentlyContinue |
            Where-Object { (Test-Path (Join-Path $_.FullName 'um\x64\kernel32.lib')) -and (Test-Path (Join-Path $_.FullName 'ucrt\x64\ucrt.lib')) } |
            Sort-Object { [version]$_.Name } -Descending | Select-Object -First 1 -ExpandProperty FullName
        if (-not $sdk) { throw 'Windows SDK x64 libraries are required. See README.md.' }
    }
    $libraries += (Join-Path $sdk 'um\x64'), (Join-Path $sdk 'ucrt\x64')
    $env:LIB = $libraries -join ';'
    & $cargo @args --manifest-path (Join-Path $projectRoot 'passthrough\Cargo.toml') --target x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) { throw "Cargo failed with exit code $LASTEXITCODE." }
} finally {
    foreach ($name in $names) { [Environment]::SetEnvironmentVariable($name, $previous[$name], 'Process') }
}
