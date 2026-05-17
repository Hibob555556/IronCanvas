$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$wasmTarget = "wasm32-unknown-unknown"
$wasmFile = Join-Path $projectRoot "target\$wasmTarget\release\iron_canvas.wasm"
$webWasmFile = Join-Path $projectRoot "web\iron_canvas.wasm"
$testTemp = Join-Path $projectRoot ".test-tmp"
$buildLog = Join-Path $testTemp "wasm-build.log"

New-Item -ItemType Directory -Force -Path $testTemp | Out-Null

Push-Location $projectRoot
try {
    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    cargo build --quiet --target $wasmTarget --release --lib > $buildLog 2>&1
    $buildExitCode = $LASTEXITCODE
    $ErrorActionPreference = $previousErrorActionPreference

    if ($buildExitCode -ne 0) {
        Get-Content $buildLog
        exit $buildExitCode
    }

    Copy-Item -Path $wasmFile -Destination $webWasmFile -Force
    Write-Host "Built web\iron_canvas.wasm"
}
finally {
    Pop-Location
}
