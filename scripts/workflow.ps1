param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"


pwsh init.ps1

. ../../spiral/scripts/core.ps1

$bundle = (Resolve-Path ../../spiral/apps/compiler/tmp).Path

{ pwsh compiler-contracts/test-dev-workflow.ps1 } | Invoke-Block -Location ..
{ pwsh compiler-contracts/test-selected-build.ps1 } | Invoke-Block -Location ..
{ pwsh compiler-contracts/test-binary-publication.ps1 } | Invoke-Block -Location ..

{ pwsh "$bundle/scripts/install-dotnet.ps1" } | Invoke-Block
{ pwsh "$bundle/scripts/build.ps1" -Mode single-flight } | Invoke-Block
{ pwsh compiler-contracts/test-attestation.ps1 } | Invoke-Block -Location ..
{ pwsh "$bundle/scripts/test-rust-exports.ps1" } | Invoke-Block

. "$bundle/scripts/env.ps1"
$env:EOIE_DOTNET = Resolve-SpiralDotnet
$env:EOIE_SPIRAL_COMPILE = Get-SpiralCompilerDll 'single-flight'
$env:EOIE_SPIRAL_BUNDLE = $bundle

{ pwsh build.ps1 -Test -CompilerContracts } | Invoke-Block -Location ..
{ pwsh compiler-contracts/test-native-probes.ps1 } | Invoke-Block -Location ..
{ pwsh compiler-contracts/test-regeneration.ps1 -IncludeAuxiliary -CargoCheck -Test -CompilerContracts } | Invoke-Block -Location ..
