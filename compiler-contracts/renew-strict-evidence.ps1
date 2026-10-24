[CmdletBinding()]
param(
    [string]$EoieRoot = (Join-Path $PSScriptRoot '..'),
    [string]$LeaseTitle = 'Strict evidence renewal',
    [string]$SpiralCompiler,
    [ValidateRange(1, 56)][int]$ParallelChains = 3,
    [string]$GrowthReason,
    [string]$GrowthTask,
    [ValidateRange(1, 86400000)][int]$TestTimeoutMs = 2700000,
    [switch]$Test
)
$ErrorActionPreference = 'Stop'
. $PSScriptRoot/spiral-compiler.ps1
$EoieRoot = (Resolve-Path -LiteralPath $EoieRoot).Path
$suffix = if ($IsWindows) { '.exe' } else { '' }
$eoie = Join-Path $EoieRoot "eoie$suffix"
$work = Join-Path $EoieRoot ('.cache/strict-renewal/' + [guid]::NewGuid().ToString('N'))
[void][IO.Directory]::CreateDirectory($work)
if (-not $SpiralCompiler) { $SpiralCompiler = Get-SpiralCompilerDll 'single-flight' }
$SpiralCompiler = Copy-EoieCompilerSnapshot -Compiler (Resolve-Path -LiteralPath $SpiralCompiler).Path -Destination (Join-Path $work 'compiler')
$env:SPIRAL_COMPILER_DLL = $SpiralCompiler
$env:EOIE_SPIRAL_COMPILE = $SpiralCompiler
$env:EOIE_DOTNET = Resolve-SpiralDotnet
$env:EOIE_SPIRAL_BUNDLE = $compilerRoot
$closeoutGateNames = @{
    CloseoutState = 'state'; CloseoutWorkspace = 'workspace'; CloseoutContracts = 'contracts'
    CloseoutBundle = 'bundle'; CloseoutColdReplay = 'cold'; CloseoutMigration = 'migration'
}

function Write-Step([string]$Name) { Write-Host "strict renewal: $(Get-Date -Format HH:mm:ss) $Name" }
function Invoke-Checked([string]$Name, [scriptblock]$Command) {
    & $Command | Out-Host
    if ($LASTEXITCODE -ne 0) { throw "strict renewal: $Name failed (exit $LASTEXITCODE)" }
}
function Get-EoieDigest([string]$Root, [string]$Relative) {
    $digest = & $eoie proxy hash $Root $Relative 2>$null
    if ($LASTEXITCODE -ne 0) { throw "strict renewal: cannot hash $Relative in $Root" }
    "$digest".Trim()
}
function Get-CloseoutArguments([string]$Release) {
    $receiptRelative = 'state/release_closeout.spi'
    $receipt = [IO.File]::ReadAllText((Join-Path $Release $receiptRelative))
    $migration = [regex]::Match($receipt, 'inl expected_migration \(\) : u32 = (\d+)u32').Groups[1].Value
    $gates = @(foreach ($gate in [regex]::Matches($receipt, '= (Closeout\w+) \("([^"]+)", "[0-9a-f]{64}"\)')) {
        $relative = $gate.Groups[2].Value
        "$($closeoutGateNames[$gate.Groups[1].Value])|$relative|$(Get-EoieDigest $Release $relative)"
    })
    if (-not $migration -or $gates.Count -ne $closeoutGateNames.Count) {
        throw "strict renewal: $receiptRelative must name the expected migration and $($closeoutGateNames.Count) gates (found $($gates.Count))"
    }
    @($Release, $receiptRelative, $migration, (Get-EoieDigest $Release $receiptRelative)) + $gates
}

$unrenewableDiagnostic = 'is forbidden|symlink|topology limit exceeded'

Push-Location -LiteralPath ([IO.Path]::GetTempPath())
try {
    Write-Step 'structural preflight'
    $baseline = @(& (Join-Path $PSScriptRoot 'test-strict-preflight.ps1') -EoieRoot $EoieRoot -PassThru |
        Where-Object { $_.PSObject.Properties.Name -contains 'Diagnostics' })
    $unrenewable = @($baseline | ForEach-Object { $_.Diagnostics } | Where-Object { $_ -match $unrenewableDiagnostic })
    if ($unrenewable.Count) { throw "strict renewal: renewal cannot repair these diagnostics, fix the tree first:`n$($unrenewable -join "`n")" }
    if ($Test) {
        Write-Step 'build.ps1 -Test -CompilerContracts'
        Invoke-Checked 'build' { & pwsh -NoProfile -File (Join-Path $EoieRoot 'build.ps1') -Test -CompilerContracts -Offline -TimeoutMs $TestTimeoutMs }
    }
    Write-Step 'agile begin'
    Invoke-Checked 'agile begin' { & $eoie agile begin $EoieRoot $LeaseTitle }
    if (-not ($GrowthReason -and $GrowthTask)) {
        $recorded = [regex]::Match([IO.File]::ReadAllText((Join-Path $EoieRoot 'state/owner_growth.spi')), 'OwnerChange \("[^"]+", "([^"]+)", "([^"]+)"\)')
        if (-not $recorded.Success) { throw 'strict renewal: state/owner_growth.spi has no change row to carry forward; pass -GrowthReason and -GrowthTask' }
        $GrowthReason = $recorded.Groups[1].Value
        $GrowthTask = $recorded.Groups[2].Value
    }
    Write-Step "owner growth receipt reason=$GrowthReason task=$GrowthTask"
    Invoke-Checked 'growth receipt' { & $eoie bundle growth-receipt $EoieRoot $GrowthReason $GrowthTask }
    $chains = $ParallelChains
    while ($true) {
        Write-Step "renew-cold-rebuild -Apply -ParallelChains $chains"
        try {
            & (Join-Path $PSScriptRoot 'renew-cold-rebuild.ps1') -EoieRoot $EoieRoot -Apply -ParallelChains $chains | Out-Host
            break
        } catch {
            if ($chains -le 1 -or "$_" -notmatch 'cold rebuild budget exceeded') { throw }
            Write-Host "strict renewal: $_; retrying with fewer parallel chains"
            $chains = [Math]::Max(1, [int][Math]::Floor($chains / 2))
        }
    }
    Write-Step 'agile check --compiler'
    Invoke-Checked 'agile check' { & $eoie agile check $EoieRoot --compiler $SpiralCompiler }
    Write-Step 'stage release root'
    $staged = @(& (Join-Path $PSScriptRoot 'test-strict-preflight.ps1') -EoieRoot $EoieRoot -PassThru |
        Where-Object { $_.PSObject.Properties.Name -contains 'ReleaseRoot' })
    if ($staged.Count -ne 1) { throw 'strict renewal: expected one staged release root' }
    $release = $staged[0].ReleaseRoot
    $closeout = Get-CloseoutArguments $release
    Write-Step 'release-closeout preview'
    Invoke-Checked 'release-closeout preview' { & $eoie proxy release-closeout preview @closeout }
    Write-Step 'release-closeout apply'
    Invoke-Checked 'release-closeout apply' { & $eoie proxy release-closeout apply @closeout }
    foreach ($renewed in Get-ChildItem -LiteralPath (Join-Path $release 'state') -File) {
        $live = Join-Path $EoieRoot "state/$($renewed.Name)"
        if ((Test-Path -LiteralPath $live) -and (Get-FileHash -LiteralPath $renewed.FullName).Hash -ceq (Get-FileHash -LiteralPath $live).Hash) { continue }
        Copy-Item -LiteralPath $renewed.FullName -Destination $live -Force
        Write-Host "strict renewal: renewed state/$($renewed.Name)"
    }
    Write-Step 'strict preflight -RequireReady'
    & (Join-Path $PSScriptRoot 'test-strict-preflight.ps1') -EoieRoot $EoieRoot -RequireReady
    Write-Host "strict renewal: eoie$suffix sha256=$((Get-FileHash -LiteralPath $eoie).Hash.ToLowerInvariant()) compiler=$SpiralCompiler work=$work"
} finally { Pop-Location }
