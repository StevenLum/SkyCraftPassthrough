param([switch]$Offline, [switch]$PackageOnly)
& (Join-Path $PSScriptRoot 'build-step5b.ps1') -Offline:$Offline -PackageOnly:$PackageOnly
