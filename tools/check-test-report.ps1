param(
    [string]$Report
)
# Reads a collected scene report (the newest one unless -Report names a folder) and prints,
# feature by feature, what the logs show: how often each thing happened and the first line of
# evidence. It reports evidence only; it cannot tell whether something felt right in play.
# Run .\tools\collect-scene-logs.ps1 first.
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
if (-not $Report) {
    $newest = Get-ChildItem -LiteralPath (Join-Path $projectRoot 'logs') -Directory -Filter 'scene-report-*' |
        Sort-Object Name -Descending | Select-Object -First 1
    if (-not $newest) { throw 'No collected report found. Run .\tools\collect-scene-logs.ps1 first.' }
    $Report = $newest.FullName
}
if (-not (Test-Path -LiteralPath $Report -PathType Container)) { throw "Not a report folder: $Report" }

function Read-Log([string]$name) {
    $path = Join-Path $Report $name
    if (Test-Path -LiteralPath $path -PathType Leaf) { return ,@(Get-Content -LiteralPath $path) }
    return ,@()
}
# Lines of $lines that match $pattern, always as an array.
function Select-Lines($lines, [string]$pattern) { return ,@($lines | Where-Object { $_ -cmatch $pattern }) }
function Show-Count([string]$label, $found) {
    Write-Host ('  {0,-52} {1,6}' -f $label, $found.Count)
    if ($found.Count -gt 0) {
        $line = [string]$found[0]
        $at = $line.IndexOf('event=')
        if ($at -gt 0) { $line = $line.Substring($at) }
        if ($line.Length -gt 170) { $line = $line.Substring(0, 170) + '...' }
        Write-Host "        first: $line"
    }
}
# How often each value of the first capture group of $pattern occurs, as "a=3 b=5".
function Show-Tally([string]$label, $lines, [string]$pattern) {
    $counts = @{}
    foreach ($line in $lines) { if ($line -cmatch $pattern) { $counts[$Matches[1]] = 1 + [int]$counts[$Matches[1]] } }
    $text = if ($counts.Count -eq 0) { 'none' } else { ($counts.GetEnumerator() | Sort-Object Name | ForEach-Object { "$($_.Name)=$($_.Value)" }) -join '  ' }
    Write-Host ('  {0,-52} {1}' -f $label, $text)
}
# Lowest, middle and highest of the numbers captured by $pattern.
function Show-Numbers([string]$label, $lines, [string]$pattern) {
    $values = @(foreach ($line in $lines) { if ($line -cmatch $pattern) { [double]$Matches[1] } }) | Sort-Object
    $text = if ($values.Count -eq 0) { 'none' } else { "lines=$($values.Count)  lowest=$($values[0])  middle=$($values[[int][Math]::Floor($values.Count / 2)])  highest=$($values[-1])" }
    Write-Host ('  {0,-52} {1}' -f $label, $text)
}
function Section([string]$title) { Write-Host ''; Write-Host $title }

$minecraft = Read-Log 'minecraft-scene.log'
$skyrim = Read-Log 'skyrim-scene.log'
$client = Read-Log 'minecraft-client-scene.log'
Write-Host "Report: $Report"
Write-Host "Lines: Minecraft native $($minecraft.Count), Skyrim native $($skyrim.Count), Minecraft client $($client.Count)"

Section 'Connection, switches and errors'
Show-Tally 'Protocol, Minecraft side' $minecraft 'event=NATIVE_READY protocol=(\d+)'
Show-Tally 'Protocol, Skyrim side' $skyrim 'event=NATIVE_READY protocol=(\d+)'
Show-Tally 'Connection states, Skyrim side' $skyrim 'event=LINK state=(\w+)'
Show-Count 'Switches read, Skyrim side' (Select-Lines $skyrim 'event=SCENE_SWITCHES')
Show-Count 'Switches read, Minecraft side' (Select-Lines $minecraft 'event=SCENE_SWITCHES')
$problems = @()
$problems += Select-Lines $skyrim 'event=(ERROR|LOAD_FAILED|\w*DISABLED)'
$problems += Select-Lines $minecraft 'event=(ERROR|\w*_ERROR|\w*DISABLED)'
$problems += Select-Lines $client 'event=\w*(DISABLED|_ERROR)|Passthrough (sender disabled|native library could not)'
# A feature that was never available says so with state=disabled or reason=missing_address.
$problems += Select-Lines $skyrim 'state=(disabled|hostility_unavailable)|reason=missing_address'
Write-Host ('  {0,-52} {1,6}' -f 'Error and "disabled" lines', $problems.Count)
foreach ($line in ($problems | Select-Object -First 12)) {
    $text = [string]$line; if ($text.Length -gt 200) { $text = $text.Substring(0, 200) + '...' }
    Write-Host "        $text"
}

Section 'Movement (0.10.37 cell borders, 0.10.38 server trust, 0.10.39 pose, 0.10.41 look-ahead)'
Show-Count 'Cell borders crossed without letting go' (Select-Lines $skyrim 'event=SCENE_ALIGNMENT state=cell_crossed')
Show-Count 'Times Skyrim let go of the character' (Select-Lines $skyrim 'event=RELEASED')
Show-Tally 'Holds in Minecraft, by reason' $client 'event=SCENE_BRIDGE_MOTION side=client state=(\w+)'
Show-Tally 'Holds on the local server (none expected)' $client 'event=SCENE_BRIDGE_MOTION side=server state=(\w+)'
Show-Count 'Steps Minecraft itself complained about' (Select-Lines $client 'moved wrongly|moved too quickly')
Show-Count 'Skyrim shapes overlapping the standing player' (Select-Lines $client 'event=SCENE_OVERLAP')
Show-Tally '  pose during those overlaps' $client 'event=SCENE_OVERLAP .* pose=(\w+)'
Show-Count 'Collision scans thrown away' (Select-Lines $skyrim 'event=SCENE_WORLD_SCAN .*active=false')

Section 'E key and Skyrim crosshair (0.10.19)'
Show-Tally 'Crosshair kinds seen (6 = enemy)' $skyrim 'event=SCENE_CROSSHAIR handle=\d+ kind=(\d+)'
Show-Tally 'Activations sent, by key' $client 'event=SCENE_INTERACT_SEND .*source=(\w+)'
Show-Tally 'Activations in Skyrim, by outcome' $skyrim 'event=SCENE_INTERACT_RECV .*state=(\w+)'

Section 'Seats, and what each E or R press met (0.10.47)'
Write-Host '  Record types: 0x18 activator, 0x1C container, 0x1D door, 0x28 furniture, 0x2B character'
Show-Count 'Seat targets under the crosshair' (Select-Lines $skyrim 'event=SCENE_CROSSHAIR .* seat=true')
Show-Tally 'Presses by the record type they met' $skyrim 'event=SCENE_INTERACT_TARGET .* base_type=(\w+)'
Show-Tally 'Locks met by those presses' $skyrim 'event=SCENE_INTERACT_TARGET .* lock=(\w+)'
Show-Tally 'Skyrim character''s sit state at those presses' $skyrim 'event=SCENE_INTERACT_TARGET .* player_sit=(\w+)'
Show-Tally 'Sit states the moved character went through' $skyrim 'event=SCENE_SEAT state=\d+ name=(\w+)'
Show-Tally 'Automatic stand-up calls and outcomes (0.10.48)' $skyrim 'event=SCENE_STAND_UP state=(\w+)'
Show-Tally 'Intentional sitting and exit (0.10.50)' $skyrim 'event=SCENE_SITTING state=(\w+)'
Show-Tally 'Furniture landing (0.10.52)' $skyrim 'event=SCENE_FURNITURE_GROUND state=(\w+)'
Show-Tally 'Native first-person model (0.10.54)' $skyrim 'event=SCENE_NATIVE_HANDS state=(\w+)'
Show-Tally 'Recovery terrain checks (0.10.51)' $skyrim 'event=SCENE_RECOVERY state=(terrain_\w+)'

Section 'Skyrim menus (0.10.20 keys, 0.10.31 W A S D and repeat)'
Write-Host '  Commands: 1-4 directions, 5 confirm, 6 back, 7 quicksave, 8 quickload, 9 take all,'
Write-Host '            10 click, 11 character menu, 12 map, 13 journal, 14 wait, 15 items,'
Write-Host '            16 magic, 17 favourites, 18 shout or power'
Show-Tally 'Commands sent, by number' $client 'event=SCENE_MENU_SEND sequence=\d+ action=(\d+)'
Show-Tally 'Commands sent, by source' $client 'event=SCENE_MENU_SEND .*source=(\w+)'
Show-Tally 'Menu-opening commands in Skyrim, by outcome' $skyrim 'event=SCENE_MENU_RECV sequence=\d+ action=1[1-7] .*state=(\w+)'
Show-Tally 'Shout or power key in Skyrim, by outcome' $skyrim 'event=SCENE_MENU_RECV sequence=\d+ action=18 .*state=(\w+)'
Show-Tally 'Left-click routes (0.10.55)' $skyrim 'event=SCENE_MENU_CLICK .*route=(\w+)'
Show-Tally 'Left-click menus (0.10.55+)' $skyrim 'event=SCENE_MENU_CLICK .*menu=(\w+)'
Show-Tally 'Passive render menus skipped (0.10.56)' $skyrim 'event=SCENE_MENU_CLICK .*passive_skipped=(\d+)'
Show-Tally 'All commands in Skyrim, by outcome' $skyrim 'event=SCENE_MENU_RECV .*state=(\w+)'
Show-Count 'Skyrim menu opened or closed' (Select-Lines $skyrim 'event=SCENE_MENU_STATE')

Section 'Skyrim hits on you (0.10.21 pool, 0.10.22 pause protection)'
Show-Count 'Hits Skyrim handed over' (Select-Lines $skyrim 'event=SCENE_DAMAGE_CAPTURE')
Show-Count 'Hits added to the pool' (Select-Lines $client 'event=SCENE_DAMAGE_POOL epoch=\d+ sequence=')
Show-Count 'Pools dropped as too old' (Select-Lines $client 'event=SCENE_DAMAGE_POOL .*state=expired')
Show-Tally 'Deliveries, taken by Minecraft or not' $client 'event=SCENE_DAMAGE_APPLY .*accepted=(\w+)'
Show-Numbers 'Hits per delivery' $client 'event=SCENE_DAMAGE_APPLY .* hits=(\d+)'
Show-Tally 'Pause protection' $skyrim 'event=SCENE_DAMAGE_SHIELD state=(\w+)'
Show-Tally 'Hits left to Skyrim, by reason' $skyrim 'event=SCENE_DAMAGE_PASSTHROUGH .*reason=(\w+)'

Section 'Skyrim healing (0.10.30)'
Show-Count 'Positive health changes Skyrim reported' (Select-Lines $skyrim 'event=SCENE_DAMAGE_CLAMP requested=[0-9]')
Show-Count 'Native restoration hooks installed (0.10.53)' (Select-Lines $skyrim 'event=SCENE_HEAL_HOOK')
Show-Count 'Native restore requests (0.10.53)' (Select-Lines $skyrim 'event=SCENE_HEAL_REQUEST')
Show-Count 'Restores handed over by Skyrim' (Select-Lines $skyrim 'event=SCENE_HEAL_CAPTURE')
Show-Count 'Restores applied in Minecraft' (Select-Lines $client 'event=SCENE_HEAL_APPLY')

Section 'Your hits on Skyrim enemies (0.10.23 Sharpness, Strength, Weakness)'
Show-Count 'Attacks sent' (Select-Lines $client 'event=SCENE_ATTACK_SEND')
Show-Count 'Attacks with a Sharpness bonus' (Select-Lines $client 'event=SCENE_ATTACK_SEND .* sharpness=[1-9]')
Show-Count 'Attacks with Strength or Weakness' (Select-Lines $client 'event=SCENE_ATTACK_SEND .* effects=-?[1-9]')
Show-Count 'Critical attacks' (Select-Lines $client 'event=SCENE_ATTACK_SEND .* critical=true')
Show-Tally 'Attacks in Skyrim, by outcome' $skyrim 'event=SCENE_ATTACK_RECV .*state=(\w+)'
Show-Tally 'Attacks dropped in Skyrim, by reason' $skyrim 'event=SCENE_ATTACK_RECV .*state=dropped reason=(\w+)'

Section 'Bow shots (0.10.44)'
Show-Count 'Shots sent' (Select-Lines $client 'event=SCENE_SHOT_SEND')
Show-Numbers 'Draw of those shots (1 = full)' $client 'event=SCENE_SHOT_SEND .* draw=([0-9.]+)'
Show-Tally 'Shots in Skyrim, hit or miss' $skyrim 'event=SCENE_SHOT_RECV .*state=(\w+)'
Show-Tally 'Misses, by reason' $skyrim 'event=SCENE_SHOT_RECV .*state=miss reason=(\w+)'
Show-Numbers 'Distance of hits, in Skyrim units (70 = one block)' $skyrim 'event=SCENE_SHOT_RECV .*state=dispatch distance=(\d+)'

Section 'Pointer (0.10.24 monitor, 0.10.32 sensitivity)'
Show-Tally 'Window width while pointing' $client 'event=SCENE_CURSOR_SEND .* window_w=(\d+)'
Show-Tally 'Width the pointer is measured against' $client 'event=SCENE_CURSOR_SEND .* span_w=(\d+)'
Show-Tally 'Speed factor' $client 'event=SCENE_CURSOR_SEND .* factor=([0-9.]+)'

Section 'Travel and height (0.10.25, 0.10.33)'
Show-Tally 'Arrivals, inside the building range or not' $client 'event=SCENE_RECOVERY state=received .*buildable=(\w+)'
Show-Tally 'Recovery steps' $client 'event=SCENE_RECOVERY state=(\w+)'
Show-Count 'Places seen (cell or world changes)' (Select-Lines $client 'event=SCENE_LOCATION_RECV')

Section 'Death and saves'
Show-Tally 'Death handling in Skyrim' $skyrim 'event=SCENE_DEATH\w* .*state=(\w+)'

Section 'Cost per frame, in microseconds (0.10.34)'
Show-Numbers 'Skyrim: average per frame' $skyrim 'event=SCENE_COST side=skyrim .* avg_us=(\d+)'
Show-Numbers 'Skyrim: worst frame' $skyrim 'event=SCENE_COST side=skyrim .* max_us=(\d+)'
Show-Numbers 'Skyrim: collision scan, ray casting per scan' $skyrim 'event=SCENE_WORLD_SCAN .* cast_us=(\d+)'
Show-Numbers 'Skyrim: frames per collision scan' $skyrim 'event=SCENE_WORLD_SCAN .* scan_frames=(\d+)'
Show-Tally 'Skyrim: scans by pace (0.10.40; calm = standing still)' $skyrim 'event=SCENE_WORLD_SCAN .* pace=(\w+)'
Show-Count 'Skyrim: boxes sent again during calm scans' (Select-Lines $skyrim 'event=SCENE_WORLD_RESEND')
Show-Numbers 'Minecraft: average per frame' $client 'event=SCENE_COST side=minecraft .* avg_us=(\d+)'
Show-Numbers 'Minecraft: worst frame' $client 'event=SCENE_COST side=minecraft .* max_us=(\d+)'
Show-Numbers 'Minecraft: sending, per frame' $client 'event=SCENE_COST side=minecraft .* send_us=(\d+)'
Show-Numbers 'Minecraft: receiving and collision, per frame' $client 'event=SCENE_COST side=minecraft .* world_us=(\d+)'
Show-Numbers 'Minecraft: hand and HUD capture, per frame' $client 'event=SCENE_COST side=minecraft .* overlay_us=(\d+)'
Show-Numbers 'Minecraft: block capture, per frame' $client 'event=SCENE_COST side=minecraft .* blocks_us=(\d+)'

Section 'Raw log size (0.10.26)'
$raw = Join-Path $projectRoot 'logs\step2'
foreach ($role in @('minecraft', 'skyrim')) {
    $latest = Get-ChildItem -LiteralPath $raw -Filter "$role-*.log" -File -ErrorAction SilentlyContinue |
        Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if ($latest) { Write-Host ('  {0,-52} {1:N1} MB  ({2})' -f "Newest $role raw log", ($latest.Length / 1MB), $latest.Name) }
}
