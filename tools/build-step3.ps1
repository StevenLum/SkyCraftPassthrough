param([switch]$Offline)
Write-Warning 'Step 3 has been superseded. Building the current step 4 packages.'
& (Join-Path $PSScriptRoot 'build-step4.ps1') -Offline:$Offline
