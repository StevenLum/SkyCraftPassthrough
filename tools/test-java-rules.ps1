# Checks the Minecraft side without starting Minecraft. A failure fails this script.
# 1. Rules: compiles BridgeRules.java (which uses nothing from Minecraft) with its worked
#    examples in minecraft\diagnostics\RulesCheck.java and runs them.
# 2. Hooks: checks every hook the mod places into Minecraft (the "mixin" files) against the
#    class files of the Minecraft jar the build uses. The build itself does not notice a
#    misspelt or renamed hook target; Minecraft would stop while loading.
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$jdk = Get-ChildItem 'C:\Program Files\Eclipse Adoptium' -Directory |
    Where-Object Name -Like 'jdk-25*' | Sort-Object Name -Descending | Select-Object -First 1
if (-not $jdk) { throw 'JDK 25 is required.' }
$javac = Join-Path $jdk.FullName 'bin\javac.exe'
$java = Join-Path $jdk.FullName 'bin\java.exe'
$classes = Join-Path $projectRoot 'minecraft\build\rules-check'
if (Test-Path -LiteralPath $classes) { Remove-Item -LiteralPath $classes -Recurse -Force }
New-Item -ItemType Directory -Force $classes | Out-Null
& $javac -d $classes `
    (Join-Path $projectRoot 'minecraft\src\main\java\dev\passthrough\BridgeRules.java') `
    (Join-Path $projectRoot 'minecraft\diagnostics\RulesCheck.java') `
    (Join-Path $projectRoot 'minecraft\diagnostics\MixinTargetsCheck.java')
if ($LASTEXITCODE -ne 0) { throw 'The Minecraft-side checks did not compile.' }
& $java -cp $classes RulesCheck
if ($LASTEXITCODE -ne 0) { throw 'A Minecraft-side rule check failed; nothing was built.' }

$properties = Get-Content -Raw -LiteralPath (Join-Path $projectRoot 'minecraft\build.gradle')
if ($properties -notmatch "com\.mojang:minecraft:([0-9.]+)") { throw 'Cannot read the Minecraft version from minecraft\build.gradle.' }
$jarName = "minecraft-merged-deobf-$($Matches[1]).jar"
$jar = Get-ChildItem -LiteralPath (Join-Path $projectRoot '.tools\gradle-cache\caches\fabric-loom') -Recurse -Filter $jarName -File -ErrorAction SilentlyContinue |
    Select-Object -First 1
if (-not $jar) { throw "Cannot find $jarName in the project's Gradle cache; build the Minecraft side once, then run this again." }
& $java -cp $classes MixinTargetsCheck `
    (Join-Path $projectRoot 'minecraft\src\main\java\dev\passthrough\mixin') `
    (Join-Path $projectRoot 'minecraft\src\main\resources\passthrough.mixins.json') `
    $jar.FullName
if ($LASTEXITCODE -ne 0) { throw 'A hook into Minecraft does not match the Minecraft jar; nothing was built.' }
