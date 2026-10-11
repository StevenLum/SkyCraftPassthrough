param(
    [string]$MinecraftLog = 'C:\Users\Admin\AppData\Roaming\PrismLauncher\instances\26.3\.minecraft\logs\latest.log'
)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$bridgeLogs = Join-Path $projectRoot 'logs\step2'
$sources = @()
foreach ($role in @('minecraft','skyrim')) {
    $latest = Get-ChildItem -LiteralPath $bridgeLogs -Filter "$role-*.log" -File |
        Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $latest) { throw "No $role bridge log found." }
    $sources += $latest.FullName
}
if (-not (Test-Path -LiteralPath $MinecraftLog -PathType Leaf)) { throw "Missing Minecraft log: $MinecraftLog" }
$sources += $MinecraftLog
$output = Join-Path $projectRoot ('logs\scene-report-' + (Get-Date -Format 'yyyyMMdd-HHmmss'))
New-Item -ItemType Directory -Path $output | Out-Null
# Scene diagnostics plus the few lifecycle and error lines that explain a silent bridge.
# Per-frame position and collision lines (SEND, RECV, BOX_*, NPC_*, WORLD_*) stay out.
$bridgePattern = 'event=(SCENE_|ERROR|LINK|RELEASED|ANCHOR|LOAD_|PLUGIN_LOADED|HOOK_INSTALLED|NATIVE_READY|OVERLAY_DISABLED|OVERLAY_ERROR)'
# Also Minecraft's own complaints about the player's movement: each one is a moment where the
# local server sent the player back, which is felt as being stuck or slowed.
$clientPattern = 'event=SCENE_|event=OVERLAY_(DISABLED|ERROR)|Passthrough (sender disabled|native library|Fabric plugin)|moved wrongly|moved too quickly|Can''t keep up'
# A disabled sender is followed by its Java stack trace; keep the first lines of it.
$clientError = 'Passthrough (sender disabled|native library)'
for ($i=0; $i -lt $sources.Count; $i++) {
    $destination = Join-Path $output (@('minecraft-scene.log','skyrim-scene.log','minecraft-client-scene.log')[$i])
    if ($i -lt 2) {
        Select-String -LiteralPath $sources[$i] -Pattern $bridgePattern |
            ForEach-Object { $_.Line } | Set-Content -LiteralPath $destination -Encoding UTF8
    } else {
        Select-String -LiteralPath $sources[$i] -Pattern $clientPattern -Context 0,16 |
            ForEach-Object { $_.Line; if ($_.Line -match $clientError) { $_.Context.PostContext } } |
            Set-Content -LiteralPath $destination -Encoding UTF8
    }
    Write-Host "COLLECTED $destination"
}
Write-Host "Scene report: $output"
