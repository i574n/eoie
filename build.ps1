[CmdletBinding()]
param(
    [switch]$Test,
    [switch]$SkipRelease,
    [string[]]$Package = @(),
    [string]$TargetDirectory,
    [switch]$CompilerContracts,
    [switch]$Offline,
    [string]$SpiralCompiler
)
$ErrorActionPreference = 'Stop'
$targetRoot = if ($TargetDirectory) { [IO.Path]::GetFullPath($TargetDirectory, $PWD.Path) } else { Join-Path $PSScriptRoot 'src/target' }
$suffix = if ($IsWindows) { '.exe' } else { '' }
$driver = $env:EOIE_DEV_BINARY
if (-not $driver) {
    $driverTarget = Join-Path $targetRoot 'workflow-driver'
    $driver = Join-Path $driverTarget "debug/eoie-dev$suffix"
    $bootstrap = @('build', '--locked', '--package', 'eoie-dev', '--target-dir', $driverTarget)
    if ($Offline) { $bootstrap += '--offline' }
    Push-Location (Join-Path $PSScriptRoot 'src')
    try { & cargo @bootstrap; if ($LASTEXITCODE -ne 0) { throw 'Native Spiral developer workflow build failed.' } }
    finally { Pop-Location }
}
$driver = (Resolve-Path -LiteralPath $driver).Path
$arguments = @('--root', $PSScriptRoot, '--target-dir', $targetRoot)
if ($Test) { $arguments += '--test' }
if ($SkipRelease) { $arguments += '--skip-release' }
if ($Offline) { $arguments += '--offline' }
if ($CompilerContracts) { $arguments += '--compiler-contracts' }
if ($SpiralCompiler) { $arguments += @('--compiler', $SpiralCompiler) }
foreach ($name in $Package) { $arguments += @('--package', $name) }
& $driver @arguments
if ($LASTEXITCODE -ne 0) { throw "Native Spiral developer workflow failed (exit $LASTEXITCODE)." }
if ($SkipRelease) { return }
. (Join-Path $PSScriptRoot 'compiler-contracts/workspace.ps1')
$published = Publish-EoieBinary -Source (Join-Path $targetRoot "release/eoie$suffix") -Destination (Join-Path $PSScriptRoot "eoie$suffix") -ValidationRoot $PSScriptRoot
Write-Host "EOIE candidate schema and source topology checks passed; published=$published"
