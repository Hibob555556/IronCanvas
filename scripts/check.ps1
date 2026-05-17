$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$testTemp = Join-Path $projectRoot ".test-tmp"
$fmtLog = Join-Path $testTemp "fmt.log"

New-Item -ItemType Directory -Force -Path $testTemp | Out-Null

function Invoke-Step {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [scriptblock]$Command
    )

    Write-Host "==> $Name"
    & $Command

    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

Push-Location $projectRoot
try {
    Invoke-Step "Checking Rust formatting" {
        $previousErrorActionPreference = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        cargo fmt --check --quiet > $fmtLog 2>&1
        $fmtExitCode = $LASTEXITCODE
        $ErrorActionPreference = $previousErrorActionPreference

        if ($fmtExitCode -ne 0) {
            Get-Content $fmtLog
        }
        $global:LASTEXITCODE = $fmtExitCode
    }
    Invoke-Step "Running tests" { & "$PSScriptRoot\test.ps1" }
    Invoke-Step "Checking coverage" { & "$PSScriptRoot\coverage.ps1" }
}
finally {
    Pop-Location
}
