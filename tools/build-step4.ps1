param([switch]$Offline, [switch]$PackageOnly)
& (Join-Path $PSScriptRoot 'build-step5.ps1') -Offline:$Offline -PackageOnly:$PackageOnly
