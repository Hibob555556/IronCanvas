$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$testTemp = Join-Path $projectRoot ".test-tmp"

New-Item -ItemType Directory -Force -Path $testTemp | Out-Null

$env:TEMP = $testTemp
$env:TMP = $testTemp

Push-Location $projectRoot
try {
    cargo test
    & "$PSScriptRoot\test-web.ps1"
}
finally {
    Pop-Location
}
