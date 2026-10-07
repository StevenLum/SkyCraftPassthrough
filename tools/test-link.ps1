$ErrorActionPreference = 'Stop'
& (Join-Path $PSScriptRoot 'cargo.ps1') test --offline
Write-Host 'PASS: Rust connection and position checks completed. Logs: passthrough\target\link-tests and logs\step2-fixtures'
