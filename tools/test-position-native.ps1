$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
& (Join-Path $PSScriptRoot 'cargo.ps1') build --release --offline
$jdk = Get-ChildItem 'C:\Program Files\Eclipse Adoptium' -Directory |
    Where-Object Name -Like 'jdk-25*' | Sort-Object Name -Descending | Select-Object -First 1
if (-not $jdk) { throw 'JDK 25 is required.' }
$classes = Join-Path $projectRoot 'minecraft\build\native-smoke'
New-Item -ItemType Directory -Force $classes | Out-Null
& (Join-Path $jdk.FullName 'bin\javac.exe') -d $classes (Join-Path $projectRoot 'minecraft\src\main\java\dev\passthrough\NativeLink.java') (Join-Path $projectRoot 'minecraft\diagnostics\NativeSmoke.java')
if ($LASTEXITCODE -ne 0) { throw 'Java smoke check compilation failed.' }
$session = 'native_' + [guid]::NewGuid().ToString('N')
$logs = Join-Path $projectRoot 'logs\step2-fixtures'
New-Item -ItemType Directory -Force $logs | Out-Null
$received = Join-Path $logs "$session-received.log"
$executable = Join-Path $projectRoot 'passthrough\target\x86_64-pc-windows-msvc\release\position-receiver.exe'
$dll = Join-Path $projectRoot 'passthrough\target\x86_64-pc-windows-msvc\release\passthrough_link.dll'
$previousSession = $env:PASSTHROUGH_SESSION
$previousMode = $env:PASSTHROUGH_TEST_MODE
$receiver = $null
try {
    $env:PASSTHROUGH_SESSION=$session
    $env:PASSTHROUGH_TEST_MODE='1'
    $started = Get-Date
    $receiver = Start-Process -FilePath $executable -ArgumentList @($session,('"'+$received+'"')) -PassThru -WindowStyle Hidden
    & (Join-Path $jdk.FullName 'bin\java.exe') --enable-native-access=ALL-UNNAMED -cp $classes NativeSmoke $dll
    if ($LASTEXITCODE -ne 0) { throw 'Java native publisher failed.' }
    if (-not $receiver.WaitForExit(20000)) { throw 'Receiver timed out.' }
    if ($receiver.ExitCode -ne 0) { throw "Receiver failed with exit code $($receiver.ExitCode)." }
    $source = Get-ChildItem $logs -Filter 'minecraft-*.log' |
        Where-Object LastWriteTime -GE $started | Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $source) { throw 'No source log was produced.' }
    & (Join-Path $PSScriptRoot 'compare-position-logs.ps1') -MinecraftLog $source.FullName -SkyrimLog $received -Fixture
    Write-Host "Source: $($source.FullName)"
    Write-Host "Receiver: $received"
} finally {
    if ($receiver -and -not $receiver.HasExited) { Stop-Process -Id $receiver.Id -Force }
    $env:PASSTHROUGH_SESSION=$previousSession
    $env:PASSTHROUGH_TEST_MODE=$previousMode
}
