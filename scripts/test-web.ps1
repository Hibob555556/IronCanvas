$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot

Push-Location $projectRoot
try {
    & "$PSScriptRoot\build-wasm.ps1"
    node --test web/*.test.mjs
}
finally {
    Pop-Location
}
