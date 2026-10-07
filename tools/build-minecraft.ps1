param([switch]$Offline)
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$names = @('JAVA_HOME','GRADLE_USER_HOME','TEMP','TMP')
$previous = @{}
foreach ($name in $names) { $previous[$name] = [Environment]::GetEnvironmentVariable($name,'Process') }
try {
    $java25 = $env:JAVA_HOME -and (Test-Path (Join-Path $env:JAVA_HOME 'release')) -and
        ((Get-Content (Join-Path $env:JAVA_HOME 'release')) -match '^JAVA_VERSION="25[."]')
    if (-not $java25) {
        $jdk = Get-ChildItem 'C:\Program Files\Eclipse Adoptium' -Directory |
            Where-Object Name -Like 'jdk-25*' | Sort-Object Name -Descending | Select-Object -First 1
        if (-not $jdk) { throw 'JDK 25 is required; set JAVA_HOME to its installation directory.' }
        $env:JAVA_HOME = $jdk.FullName
    }
    $env:GRADLE_USER_HOME = Join-Path $projectRoot '.tools\gradle-cache'
    $env:TEMP = Join-Path $projectRoot '.tools\temp'
    $env:TMP = $env:TEMP
    $gradle = Join-Path $projectRoot '.tools\gradle-9.7.1\bin\gradle.bat'
    if (-not (Test-Path $gradle)) { throw 'Gradle 9.7.1 is required in .tools\gradle-9.7.1 (see README.md).' }
    $options = @('--no-daemon','--console=plain','--project-dir',(Join-Path $projectRoot 'minecraft'),'build')
    if ($Offline) { $options += '--offline' }
    & $gradle @options
    if ($LASTEXITCODE -ne 0) { throw "Minecraft build failed with exit code $LASTEXITCODE" }
} finally {
    foreach ($name in $names) { [Environment]::SetEnvironmentVariable($name,$previous[$name],'Process') }
}
