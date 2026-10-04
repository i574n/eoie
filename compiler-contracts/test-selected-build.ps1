[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
$src = Join-Path (Split-Path $PSScriptRoot -Parent) 'src'
$manifest = Join-Path $src 'eoie_dev/Cargo.toml'
if (-not (Test-Path -LiteralPath $manifest)) {
    throw "eoie-dev package manifest was not found at '$manifest'."
}
if (-not (Get-Command cargo -CommandType Application -ErrorAction SilentlyContinue | Select-Object -First 1)) {
    throw 'cargo is not on PATH. Install Rust 1.88 before the selected-build contracts. Command: cargo test --locked --package eoie-dev --test selected_workflow from src.'
}
Push-Location $src
try {
    & cargo test --locked --package eoie-dev --test selected_workflow
    if ($LASTEXITCODE -ne 0) { throw "Selected native build contracts failed (exit $LASTEXITCODE)." }
} finally { Pop-Location }
Write-Host 'EOIE selected-build contracts passed.'
