$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$testTemp = Join-Path $projectRoot ".test-tmp"
$rustTestLog = Join-Path $testTemp "rust-test.log"

New-Item -ItemType Directory -Force -Path $testTemp | Out-Null

$env:TEMP = $testTemp
$env:TMP = $testTemp

Push-Location $projectRoot
try {
    Write-Host "==> Rust tests"
    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    cargo test --quiet > $rustTestLog 2>&1
    $rustTestExitCode = $LASTEXITCODE
    $ErrorActionPreference = $previousErrorActionPreference

    if ($rustTestExitCode -ne 0) {
        Get-Content $rustTestLog
        exit $rustTestExitCode
    }
    Write-Host "Rust tests passed"

    Write-Host "==> Web and WASM tests"
    & "$PSScriptRoot\test-web.ps1"
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}
finally {
    Pop-Location
}
