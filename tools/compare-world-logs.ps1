param(
    [Parameter(Mandatory=$true)][string]$MinecraftLog,
    [Parameter(Mandatory=$true)][string]$SkyrimLog
)
$ErrorActionPreference = 'Stop'
function Read-WorldRows([string]$path, [string]$direction) {
    $rows = @{}
    $counts = @{}
    foreach ($line in [IO.File]::ReadLines((Resolve-Path -LiteralPath $path).Path)) {
        if ($line -notmatch "event=(WORLD|BOX|NPC)_$direction ") { continue }
        $type = $Matches[1]
        $fields = @{}
        foreach ($match in [regex]::Matches($line,'(\w+)=([^ ]+)')) { $fields[$match.Groups[1].Value] = $match.Groups[2].Value }
        $snapshot = "$($fields.sender)/$($fields.world)/$($fields.epoch)/$($fields.snapshot)"
        $suffix = if ($type -eq 'BOX') { $fields.index } elseif ($type -eq 'NPC') { $fields.id } else { '' }
        $key = "$snapshot/$type/$suffix"
        if ($rows.ContainsKey($key)) { throw "Duplicate row $key in $path" }
        $body = $line.Substring($line.IndexOf('event=')).Replace("_$direction ",'_DATA ')
        $rows[$key] = $body
        if (-not $counts.ContainsKey($snapshot)) { $counts[$snapshot] = @{ BOX=0; NPC=0; Header=$null } }
        if ($type -eq 'WORLD') { $counts[$snapshot].Header = $fields }
        else { $counts[$snapshot][$type]++ }
    }
    foreach ($entry in $counts.GetEnumerator()) {
        $value = $entry.Value
        if ($null -eq $value.Header -or $value.BOX -ne [int]$value.Header.boxes -or $value.NPC -ne [int]$value.Header.npcs) {
            throw "Incomplete snapshot $($entry.Key) in $path; stop both games before comparing."
        }
    }
    return @{ Rows=$rows; Counts=$counts }
}
$sent = Read-WorldRows $SkyrimLog 'SEND'
$received = Read-WorldRows $MinecraftLog 'RECV'
if ($received.Counts.Count -eq 0) { throw 'No received world snapshots; this is not a passing comparison.' }
$boxRows=0; $npcRows=0
foreach ($entry in $received.Rows.GetEnumerator()) {
    if (-not $sent.Rows.ContainsKey($entry.Key) -or $sent.Rows[$entry.Key] -cne $entry.Value) {
        throw "World data mismatch or missing sender row: $($entry.Key)"
    }
    if ($entry.Key -like '*/BOX/*') { $boxRows++ }
    if ($entry.Key -like '*/NPC/*') { $npcRows++ }
}
if ($boxRows -eq 0) { throw 'Snapshots matched but contain no geometry; collision export is not demonstrated.' }
Write-Host "PASS: $($received.Counts.Count) complete received snapshots, $boxRows collision boxes and $npcRows NPC records match Skyrim exactly."
if ($npcRows -eq 0) { Write-Warning 'No NPC records were received. Repeat near a loaded NPC to check that direction.' }
Write-Host 'Transport comparison only: check Minecraft latest.log for COLLISION_QUERY on both client and server, then use the manual movement procedure.'
