param(
    [int]$Keep = 2,
    [switch]$Delete
)
# Lists the old raw bridge logs in logs\step2 and, only with -Delete, removes them.
# The newest $Keep logs of each game are always kept. Collected scene reports
# (logs\scene-report-*) and everything else are never touched.
$ErrorActionPreference = 'Stop'
if ($Keep -lt 1) { throw 'Keep at least 1 log per game.' }
$projectRoot = Split-Path -Parent $PSScriptRoot
$folder = Join-Path $projectRoot 'logs\step2'
if (-not (Test-Path -LiteralPath $folder -PathType Container)) { Write-Host "No raw log folder at $folder."; return }
$old = @()
$kept = @()
foreach ($role in @('minecraft','skyrim')) {
    $logs = @(Get-ChildItem -LiteralPath $folder -Filter "$role-*.log" -File | Sort-Object LastWriteTime -Descending)
    $kept += $logs | Select-Object -First $Keep
    $old += $logs | Select-Object -Skip $Keep
}
function Format-Size([long]$bytes) { if ($bytes -ge 1GB) { '{0:N1} GB' -f ($bytes / 1GB) } else { '{0:N1} MB' -f ($bytes / 1MB) } }
$keptBytes = [long](($kept | Measure-Object Length -Sum).Sum)
if ($old.Count -eq 0) {
    Write-Host "Nothing to remove: $($kept.Count) raw logs ($(Format-Size $keptBytes)) in $folder, at most $Keep per game."
    return
}
$oldBytes = [long](($old | Measure-Object Length -Sum).Sum)
if (-not $Delete) {
    Write-Host "Would remove $($old.Count) raw log files, $(Format-Size $oldBytes), from $folder."
    Write-Host "Would keep the newest $Keep per game: $($kept.Count) files, $(Format-Size $keptBytes)."
    Write-Host 'Nothing was deleted. Run again with -Delete to remove them.'
    return
}
$removed = 0; $freed = [long]0; $failed = 0
foreach ($file in $old) {
    try {
        $size = $file.Length
        Remove-Item -LiteralPath $file.FullName -Force
        $removed++; $freed += $size
    } catch {
        # A log still open in a running game cannot be removed; leave it and say so.
        $failed++
        Write-Warning "Could not remove $($file.Name): $($_.Exception.Message)"
    }
}
Write-Host "Removed $removed raw log files, $(Format-Size $freed). Kept the newest $Keep per game."
if ($failed -gt 0) { Write-Host "$failed files could not be removed; close both games and run again." }
