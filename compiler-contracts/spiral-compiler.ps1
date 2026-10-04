# Dot-source from the EOIE compiler contracts. Loads the Spiral compiler's shared script environment
# (scripts/env.ps1: Resolve-SpiralDotnet, Get-SpiralCompilerDll, Get-SpiralCacheDir, $BundleRoot).
# The bundle is EOIE_SPIRAL_BUNDLE when set, else ../spiral/apps/compiler/tmp (Resolve-EoieSpiralBundle).
. $PSScriptRoot/workspace.ps1
$compilerRoot = Resolve-EoieSpiralBundle
. (Join-Path $compilerRoot 'scripts/env.ps1')
