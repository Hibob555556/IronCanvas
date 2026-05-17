$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$testTemp = Join-Path $projectRoot ".test-tmp"
$coverageJson = Join-Path $testTemp "coverage.json"
$coverageLog = Join-Path $testTemp "coverage.log"

New-Item -ItemType Directory -Force -Path $testTemp | Out-Null

$env:TEMP = $testTemp
$env:TMP = $testTemp

Push-Location $projectRoot
try {
    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    cargo llvm-cov --quiet --json --summary-only --fail-under-lines 90 --output-path $coverageJson > $coverageLog 2>&1
    $coverageExitCode = $LASTEXITCODE
    $ErrorActionPreference = $previousErrorActionPreference

    if ($coverageExitCode -ne 0) {
        Get-Content $coverageLog
        exit $coverageExitCode
    }

    $coverage = Get-Content $coverageJson -Raw | ConvertFrom-Json
    $totals = $coverage.data[0].totals
    $lineCoverage = [math]::Round($totals.lines.percent, 2)
    $functionCoverage = [math]::Round($totals.functions.percent, 2)
    $regionCoverage = [math]::Round($totals.regions.percent, 2)

    Write-Host "Coverage: $lineCoverage% lines, $functionCoverage% functions, $regionCoverage% regions"
}
finally {
    Pop-Location
}
