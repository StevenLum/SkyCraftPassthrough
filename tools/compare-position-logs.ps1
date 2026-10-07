param(
    [Parameter(Mandatory=$true)][string]$MinecraftLog,
    [Parameter(Mandatory=$true)][string]$SkyrimLog,
    [switch]$Fixture,
    [switch]$RequireLook
)
$ErrorActionPreference = 'Stop'
$culture = [Globalization.CultureInfo]::InvariantCulture
function Parse-Fields([string]$line) {
    $fields = @{}
    foreach ($match in [regex]::Matches($line,'(\w+)=([^\s]+)')) { $fields[$match.Groups[1].Value]=$match.Groups[2].Value }
    return $fields
}
$sent = @{}
foreach ($line in [IO.File]::ReadLines((Resolve-Path -LiteralPath $MinecraftLog).Path)) {
    if ($line -notmatch '\bevent=SEND\b') { continue }
    $fields = Parse-Fields $line
    if ($fields.active -ne '1') { continue }
    $key = "$($fields.sender):$($fields.world):$($fields.frame)"
    if ($sent.ContainsKey($key)) { throw "Duplicate SEND identity $key" }
    $sent[$key] = $fields
}
$receiveEvent = if ($Fixture) { '\bevent=FIXTURE_RECV\b' } else { '\bevent=RECV\b' }
$count = 0
$maxError = 0.0
$lookCount = 0
$maxLookError = 0.0
foreach ($line in [IO.File]::ReadLines((Resolve-Path -LiteralPath $SkyrimLog).Path)) {
    if ($RequireLook -and $line -match '\bevent=LOOK_RECV\b') {
        $look = Parse-Fields $line
        $key = "$($look.sender):$($look.world):$($look.frame)"
        if (-not $sent.ContainsKey($key)) { throw "No SEND for look frame $key" }
        foreach ($field in @('mc_yaw','mc_pitch')) {
            if (-not $sent[$key].ContainsKey($field) -or -not $look.ContainsKey($field)) { throw "Missing $field at $key; install both 0.4.0 adapters." }
            $a = [double]::Parse($sent[$key][$field],$culture)
            $b = [double]::Parse($look[$field],$culture)
            if ([double]::IsNaN($a) -or [double]::IsInfinity($a) -or [BitConverter]::DoubleToInt64Bits($a) -ne [BitConverter]::DoubleToInt64Bits($b)) { throw "Look transport mismatch: $field at $key" }
        }
        $heading = (([double]::Parse($look.mc_yaw,$culture)-180.0)%360.0+360.0)%360.0 * [Math]::PI/180.0
        $pitch = [double]::Parse($look.mc_pitch,$culture)*[Math]::PI/180.0
        foreach ($entry in @(@('yaw',$heading),@('pitch',$pitch))) {
            $axis=$entry[0]; $expected=$entry[1]
            foreach ($prefix in @('target','actual')) {
                $value=[double]::Parse($look["${prefix}_${axis}_rad"],$culture)
                if ([double]::IsNaN($value) -or [double]::IsInfinity($value)) { throw "Invalid look angle at $key" }
                $difference=[Math]::Abs($value-$expected)
                $maxLookError=[Math]::Max($maxLookError,$difference)
                if ($difference -gt 0.00001) { throw "Look conversion/read-back mismatch at $key" }
            }
        }
        $lookCount++
    }
    if ($line -notmatch $receiveEvent) { continue }
    $received = Parse-Fields $line
    $key = "$($received.sender):$($received.world):$($received.frame)"
    if (-not $sent.ContainsKey($key)) { throw "No SEND found for received frame $key" }
    foreach ($field in @('partial','mc_x','mc_y','mc_z')) {
        $a = [double]::Parse($sent[$key][$field],$culture)
        $b = [double]::Parse($received[$field],$culture)
        if ([double]::IsNaN($a) -or [double]::IsInfinity($a) -or [BitConverter]::DoubleToInt64Bits($a) -ne [BitConverter]::DoubleToInt64Bits($b)) {
            throw "Mismatch in $field for frame ${key}: sent=$a received=$b"
        }
    }
    if (-not $Fixture) {
        foreach ($axis in @('x','y','z')) {
            $target = [double]::Parse($received["target_$axis"],$culture)
            $actual = [double]::Parse($received["actual_$axis"],$culture)
            if ([double]::IsNaN($target) -or [double]::IsInfinity($target) -or [double]::IsNaN($actual) -or [double]::IsInfinity($actual)) { throw "Non-finite puppet coordinates for frame $key" }
            $errorValue = [Math]::Abs($target-$actual)
            $maxError = [Math]::Max($maxError,$errorValue)
            if ($errorValue -gt 0.01) { throw "Puppet read-back error $errorValue Skyrim units at frame $key" }
        }
    }
    $count++
}
if ($count -eq 0) { throw 'No matching received frames were found. This is not a successful test.' }
$kind = if ($Fixture) { 'synthetic fixture' } else { 'game logs' }
Write-Host "PASS: $count matching received frames ($kind); all Minecraft coordinates and partial ticks match exactly."
if (-not $Fixture) { Write-Host "Maximum Skyrim target/read-back difference: $maxError units." }
Write-Host "Sent frames: $($sent.Count). A latest-value connection may skip intermediate frames."
if ($RequireLook) {
    if ($lookCount -eq 0) { throw 'No first-person LOOK_RECV frames; camera look is not verified.' }
    Write-Host "PASS: $lookCount look frames match; maximum conversion/actor read-back difference $maxLookError radians."
    Write-Host 'Actor angle check only. Camera-state quaternions are sampled before the next camera update; confirm visible camera response manually.'
}
