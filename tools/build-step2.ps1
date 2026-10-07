param([switch]$Offline)
# Keep the old documented command usable without mixing old Java and new native code.
Write-Warning 'Step 2 has been superseded. Building the current step 3 packages.'
& (Join-Path $PSScriptRoot 'build-step3.ps1') -Offline:$Offline
