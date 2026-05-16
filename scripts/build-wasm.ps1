$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$wasmTarget = "wasm32-unknown-unknown"
$wasmFile = Join-Path $projectRoot "target\$wasmTarget\release\iron_canvas.wasm"
$webWasmFile = Join-Path $projectRoot "web\iron_canvas.wasm"

Push-Location $projectRoot
try {
    cargo build --target $wasmTarget --release --lib
    Copy-Item -Path $wasmFile -Destination $webWasmFile -Force
    Write-Host "Built web\iron_canvas.wasm"
}
finally {
    Pop-Location
}
