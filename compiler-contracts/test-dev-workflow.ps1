[CmdletBinding()]
param([string]$DevBinary)
$ErrorActionPreference = 'Stop'
$eoiRoot = Split-Path $PSScriptRoot -Parent
$fixture = Join-Path ([IO.Path]::GetTempPath()) ('eoie dev contracts ' + [guid]::NewGuid().ToString('N'))
$root = Join-Path $fixture 'workspace with spaces'
$bundle = Join-Path $fixture 'compiler with spaces'
$targetDirectory = Join-Path $root 'src/target/shared validation'
$pwsh = (Get-Process -Id $PID).Path
$previousMutation = $env:EOIE_TEST_MUTATE_CHECKOUT
$previousDriver = $env:EOIE_DEV_BINARY
if (-not $DevBinary) { $DevBinary = Join-Path $eoiRoot ('src/target/dev-validation/debug/eoie-dev' + $(if ($IsWindows) { '.exe' } else { '' })) }
$DevBinary = (Resolve-Path -LiteralPath $DevBinary).Path
function Write-Fixture([string]$Path, [string]$Text) {
    [void][IO.Directory]::CreateDirectory((Split-Path $Path -Parent))
    [IO.File]::WriteAllText($Path, $Text, [Text.UTF8Encoding]::new($false))
}
function Assert-Contract([bool]$Condition, [string]$Message) {
    if (-not $Condition) { throw $Message }
}
function Invoke-Regeneration([string]$Label, [bool]$Success, [string[]]$Extra = @(), [string]$Diagnostic = '') {
    $output = & $pwsh -NoProfile -File (Join-Path $PSScriptRoot 'test-regeneration.ps1') `
        -EoieRoot $root -CompilerBundle $bundle -Package alpha -Offline -TargetDirectory $targetDirectory @Extra 2>&1
    $code = $LASTEXITCODE
    if (($code -eq 0) -ne $Success) { throw "$Label failed (exit $code):`n$($output -join "`n")" }
    if ($Diagnostic -and -not ($output -join "`n").Contains($Diagnostic)) { throw "$Label missed expected diagnostic '$Diagnostic':`n$($output -join "`n")" }
    if ($Success) { $script:lastStagedEoie = [regex]::Match(($output -join "`n"), 'Staged workspace: (.+)').Groups[1].Value.Trim() }
    Write-Host "PASS $Label"
}
try {
    $env:EOIE_DEV_BINARY = $DevBinary
    . (Join-Path $PSScriptRoot 'workspace.ps1')
    $managed = Join-Path $fixture 'managed compiler/Compiler.DLL'
    Write-Fixture $managed 'first compiler'
    Write-Fixture (Join-Path (Split-Path $managed) 'runtimes/native/dependency.dll') 'dependency'
    $frozen = Copy-EoieCompilerSnapshot -Compiler $managed -Destination (Join-Path $fixture 'frozen compiler')
    Write-Fixture $managed 'replacement compiler'
    Assert-Contract ([IO.File]::ReadAllText($frozen) -ceq 'first compiler') 'Compiler snapshot changed with its source.'
    Assert-Contract ([IO.File]::ReadAllText((Join-Path (Split-Path $frozen) 'runtimes/native/dependency.dll')) -ceq 'dependency') 'Compiler runtime dependency was not snapshotted.'
    Assert-Contract (Test-Path -LiteralPath (Join-Path (Split-Path $frozen) 'eoie-compiler-snapshot.json')) 'Compiler identity manifest is missing.'
    Write-Host 'PASS managed compiler snapshot survives a later rebuild'
    $generatedInput = Join-Path $fixture 'generated input.rs'
    $generatedOutput = Join-Path $fixture 'generated output.rs'
    Write-Fixture $generatedInput "first`r`nsecond`r`n"
    Assert-Contract (Set-EoieGeneratedFile -Source $generatedInput -Destination $generatedOutput) 'First generated publication was not written.'
    Assert-Contract ([IO.File]::ReadAllText($generatedOutput) -ceq "first`nsecond`n") 'Generated line endings were not normalized.'
    $generatedStamp = [IO.File]::GetLastWriteTimeUtc($generatedOutput)
    Assert-Contract (-not (Set-EoieGeneratedFile -Source $generatedInput -Destination $generatedOutput)) 'Equivalent CRLF output was rewritten.'
    Assert-Contract ([IO.File]::GetLastWriteTimeUtc($generatedOutput) -eq $generatedStamp) 'Equivalent normalized output timestamp changed.'
    Write-Host 'PASS generated line endings are stable across platforms'
    Write-Fixture (Join-Path $root 'src/Cargo.toml') @'
[workspace]
members = ["alpha", "broken"]
resolver = "3"
'@
    Write-Fixture (Join-Path $root 'src/alpha/Cargo.toml') @'
[package]
name = "alpha"
version = "0.0.0"
edition = "2024"
[lib]
path = "generated.rs"
[package.metadata.spiral]
entry = "custom_entry.spi"
output = "generated.rs"
[[package.metadata.spiral.auxiliary]]
entry = "aux_entry.spi"
output = "probe.rs"
[[test]]
name = "probe"
path = "probe.rs"
'@
    Write-Fixture (Join-Path $root 'src/broken/Cargo.toml') @'
[package]
name = "broken"
version = "0.0.0"
edition = "2024"
[lib]
path = "broken.rs"
'@
    Write-Fixture (Join-Path $root 'src/broken/broken.rs') 'compile_error!("unselected package must not build");'
    $source = Join-Path $root 'src/alpha/custom_entry.spi'
    $target = Join-Path $root 'src/alpha/generated.rs'
    Write-Fixture $source 'good'
    Write-Fixture $target 'pub fn value() -> i32 { 1 }'
    Write-Fixture (Join-Path $root 'src/alpha/aux_entry.spi') 'good'
    Write-Fixture (Join-Path $root 'src/alpha/probe.rs') 'pub fn probe() {}'
    Write-Fixture (Join-Path $root 'src/alpha/custom_entry.rs') 'pub fn value() -> i32 { 7 }'
    Copy-Item -LiteralPath (Join-Path $eoiRoot 'build.ps1') -Destination (Join-Path $root 'build.ps1')
    Write-Fixture (Join-Path $bundle 'scripts/env.ps1') @'
$BundleRoot = Split-Path $PSScriptRoot -Parent
function Resolve-SpiralDotnet { (Get-Process -Id $PID).Path }
function Get-SpiralCompilerDll([string]$Mode) { Join-Path $BundleRoot 'compiler.ps1' }
function Get-SpiralCacheDir { Join-Path $BundleRoot 'cache' }
'@
    Write-Fixture (Join-Path $bundle 'compiler.ps1') @'
$ErrorActionPreference = 'Stop'
$source = [IO.File]::ReadAllText($args[2])
if ($source -eq 'fail-compiler') { [Console]::Error.WriteLine('fixture compiler rejected input'); exit 7 }
if ($source -eq 'sleep-compiler') { Start-Sleep -Seconds 20; exit 8 }
$rust = if ($source -eq 'bad-rust') { 'this is invalid Rust' } else {
    '#[path="custom_entry.rs"] mod adapter; pub fn value() -> i32 { 42 } #[cfg(test)] mod tests { #[test] fn generated_value() { assert_eq!(super::value(),42); assert_eq!(super::adapter::value(),7); } }'
}
[IO.File]::WriteAllText([IO.Path]::ChangeExtension($args[2], '.rs'), $rust)
[IO.File]::WriteAllText($args[3], $rust)
if ($env:EOIE_TEST_MUTATE_CHECKOUT) { [IO.File]::AppendAllText($env:EOIE_TEST_MUTATE_CHECKOUT, ' changed concurrently') }
'@
    Push-Location (Join-Path $root 'src')
    try {
        & cargo generate-lockfile --offline
        if ($LASTEXITCODE -ne 0) { throw 'Fixture lockfile generation failed.' }
    } finally { Pop-Location }
    $before = [IO.File]::ReadAllText($target)
    Invoke-Regeneration 'isolated check with spaced paths and explicit entry' $true @('-CargoCheck')
    Assert-Contract ([IO.File]::ReadAllText($target) -ceq $before) 'Check unexpectedly published output.'
    Invoke-Regeneration 'selected package tests and publication without release' $true @('-CargoCheck', '-Test', '-SkipRelease', '-Publish')
    Assert-Contract ([IO.File]::ReadAllText($target).Contains('42')) 'Validated output was not published.'
    Assert-Contract (-not (Test-Path (Join-Path $root 'eoie.exe')) -and -not (Test-Path (Join-Path $root 'eoie'))) 'Development run installed a release binary.'
    $stamp = [DateTime]::UtcNow.AddDays(-1)
    [IO.File]::SetLastWriteTimeUtc($target, $stamp)
    $stamp = [IO.File]::GetLastWriteTimeUtc($target)
    Invoke-Regeneration 'unchanged output preserves timestamp' $true @('-CargoCheck', '-Publish')
    Assert-Contract ([IO.File]::GetLastWriteTimeUtc($target) -eq $stamp) 'Unchanged output timestamp changed.'
    Invoke-Regeneration 'auxiliary target publication with reusable spaced cache' $true @('-CargoCheck', '-Test', '-SkipRelease', '-Publish', '-IncludeAuxiliary')
    Assert-Contract ([IO.File]::ReadAllText((Join-Path $root 'src/alpha/probe.rs')).Contains('42')) 'Auxiliary output was not published.'
    Assert-Contract (Test-Path -LiteralPath (Join-Path $targetDirectory 'debug/deps')) 'Explicit Cargo target directory was not used.'
    Assert-Contract ([IO.File]::GetLastWriteTimeUtc($target) -eq $stamp) 'Auxiliary publication touched unchanged primary output.'
    Assert-Contract (-not (Test-Path -LiteralPath (Join-Path $script:lastStagedEoie 'src/alpha/aux_entry.rs'))) 'Undeclared compiler sidecar remains in staged sources.'
    Assert-Contract ([IO.File]::ReadAllText((Join-Path $script:lastStagedEoie 'src/alpha/custom_entry.rs')) -ceq 'pub fn value() -> i32 { 7 }') 'Compiler overwrote a copied native adapter.'
    $before = [IO.File]::ReadAllText($target)
    foreach ($failure in @('fail-compiler', 'bad-rust', 'sleep-compiler')) {
        Write-Fixture $source $failure
        $extra = @('-CargoCheck', '-Publish')
        if ($failure -eq 'sleep-compiler') { $extra += @('-TimeoutSec', '1') }
        $diagnostic = switch ($failure) {
            'fail-compiler' { 'fixture compiler rejected input' }
            'bad-rust' { 'Cargo rejected regenerated EOIE owners' }
            'sleep-compiler' { 'no result within 1000 ms' }
        }
        Invoke-Regeneration "$failure leaves validated output intact" $false $extra $diagnostic
        Assert-Contract ([IO.File]::ReadAllText($target) -ceq $before) "$failure replaced output."
        Assert-Contract ([IO.File]::GetLastWriteTimeUtc($target) -eq $stamp) "$failure touched output timestamp."
    }
    Write-Fixture $source 'good'
    $env:EOIE_TEST_MUTATE_CHECKOUT = $source
    Invoke-Regeneration 'concurrent source edit blocks publication' $false @('-CargoCheck', '-Publish') 'Checkout changed during validation'
    Assert-Contract ([IO.File]::ReadAllText($source).Contains('changed concurrently')) 'Concurrent-edit fixture did not run.'
    Assert-Contract ([IO.File]::GetLastWriteTimeUtc($target) -eq $stamp) 'Stale validation published output.'
    Write-Fixture $source 'good'
    $newSource = Join-Path $root 'src/alpha/new_dependency.spi'
    $env:EOIE_TEST_MUTATE_CHECKOUT = $newSource
    Invoke-Regeneration 'concurrent new source blocks publication' $false @('-CargoCheck', '-Publish') 'source file inventory differs'
    Assert-Contract (Test-Path -LiteralPath $newSource) 'New-source fixture did not run.'
    Assert-Contract ([IO.File]::GetLastWriteTimeUtc($target) -eq $stamp) 'New-source drift published output.'
    [IO.File]::Delete($newSource)
    $cacheFile = Join-Path $root '.cache/parallel-build.txt'
    [void][IO.Directory]::CreateDirectory((Split-Path $cacheFile -Parent))
    $env:EOIE_TEST_MUTATE_CHECKOUT = $cacheFile
    Invoke-Regeneration 'ignored cache additions allow unchanged publication' $true @('-CargoCheck', '-Publish')
    Assert-Contract (Test-Path -LiteralPath $cacheFile) 'Cache-addition fixture did not run.'
    Assert-Contract ([IO.File]::GetLastWriteTimeUtc($target) -eq $stamp) 'Ignored cache activity touched unchanged output.'
    Write-Host 'EOIE developer workflow contracts passed.'
} finally {
    $env:EOIE_TEST_MUTATE_CHECKOUT = $previousMutation
    $env:EOIE_DEV_BINARY = $previousDriver
    $resolvedFixture = [IO.Path]::GetFullPath($fixture)
    $temporaryRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath()).TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    if (-not $resolvedFixture.StartsWith($temporaryRoot, [StringComparison]::OrdinalIgnoreCase)) { throw "Unsafe fixture cleanup: $resolvedFixture" }
    if (Test-Path -LiteralPath $resolvedFixture) { Remove-Item -LiteralPath $resolvedFixture -Recurse -Force }
}
