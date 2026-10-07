[CmdletBinding()]
param(
    [string]$EoieRoot = (Join-Path $PSScriptRoot '..'),
    [string]$EoieBinary,
    [ValidateRange(1000, 600000)][int]$OwnerTimeoutMs = 180000,
    # Owner chains replayed at once (batch-plan's default is 12). Lower it on a shared machine: each owner's wall time is
    # the recorded max_owner_compile_ms, and contention from parallel chains inflates it.
    [ValidateRange(1, 56)][int]$ParallelChains = 12,
    [switch]$Apply
)
# ColdRebuildV1 producer (state/cold_rebuild.spi). Every recorded value is observed here, on two independent
# release-shaped roots (src, state, evidence of a verified source package, as the strict preflight stages them):
# - source a/b: `eoie proxy hash-tree <root> src` of each root;
# - product a/b: SHA-256 of eoie-cli built cold in each root (fresh target directory, CARGO_INCREMENTAL=0,
#   --remap-path-prefix=<root>=/workspace, -Cstrip=symbols and, on Windows, -Clink-arg=/Brepro, which removes the PE
#   timestamps and the PDB GUID); the two must be byte-identical;
# - owners: every declared Spiral owner (Get-EoieOwners) recompiled cold through `eoie proxy cold-rebuild-matrix` +
#   `eoie proxy batch-plan`; the slowest owner's wall time is max_owner_compile_ms;
# - global gate: the wall time of the strict release gate `eoie bundle check <root> eoie` run with product a;
# - toolchain: rustc/cargo/rustdoc/rustfmt and the Spiral compiler (its dll and the dotnet host running it) are written
#   to state/toolchain_identity.spi, and the receipt's gate hashes are taken after that.
# Without -Apply it only prints the receipt it would write. With -Apply it writes state/toolchain_identity.spi and
# state/cold_rebuild.spi and publishes product a as the root eoie binary (Publish-EoieBinary); `proxy release-closeout
# apply` then renews ColdProofV4 against it.
$ErrorActionPreference = 'Stop'
. $PSScriptRoot/workspace.ps1
$EoieRoot = (Resolve-Path -LiteralPath $EoieRoot).Path
$suffix = if ($IsWindows) { '.exe' } else { '' }
if (-not $EoieBinary) { $EoieBinary = Join-Path $EoieRoot "eoie$suffix" }
$EoieBinary = (Resolve-Path -LiteralPath $EoieBinary).Path
$work = Join-Path $EoieRoot ('.cache/cold-renewal/' + [guid]::NewGuid().ToString('N'))
[void][IO.Directory]::CreateDirectory($work)
$supervisor = Join-Path $work "supervisor$suffix"
Copy-Item -LiteralPath $EoieBinary -Destination $supervisor

function Invoke-Eoie([string[]]$Arguments) {
    $output = & $supervisor @Arguments 2>&1 | ForEach-Object { "$_" }
    if ($LASTEXITCODE -ne 0) { throw "eoie $($Arguments -join ' ') failed (exit $LASTEXITCODE): $($output -join ' | ')" }
    $output
}
function Get-Sha256([string]$Path) { (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant() }

# Two release roots from two independent verified source packages.
function New-ReleaseRoot([string]$Name) {
    $snapshot = & $PSScriptRoot/test-source-package.ps1 -EoieRoot $EoieRoot -EoieBinary $supervisor -SkipBuild -PassThru | ForEach-Object {
        if ($_.PSObject.Properties.Name -contains 'Manifest') { $_ } else { Write-Host $_ }
    }
    if (-not $snapshot -or $snapshot.Count -gt 1) { throw 'Expected one verified source snapshot.' }
    $manifest = Get-Content -Raw -LiteralPath $snapshot.Manifest | ConvertFrom-Json
    $release = Join-Path $work $Name
    foreach ($file in @($manifest | Where-Object { $_.Path -match '^(src|state|evidence)/' -and $_.Path -ne 'src/.gitignore' })) {
        $destination = Join-Path $release $file.Path
        [void][IO.Directory]::CreateDirectory((Split-Path $destination -Parent))
        Copy-Item -LiteralPath (Join-Path $snapshot.RestoredRoot $file.Path) -Destination $destination
        if ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash -cne $file.SHA256) { throw "Staging hash mismatch: $($file.Path)" }
    }
    [void][IO.Directory]::CreateDirectory((Join-Path $release 'evidence/coverage'))
    $release
}
function Get-TreeSha([string]$Root) {
    $line = Invoke-Eoie @('proxy', 'hash-tree', $Root, 'src') | Where-Object { $_ -match '[0-9a-f]{64}' } | Select-Object -Last 1
    [regex]::Match($line, '[0-9a-f]{64}').Value
}
function Build-ColdProduct([string]$Root) {
    $target = "$Root-target"
    $flags = "--remap-path-prefix=$Root=/workspace -Cstrip=symbols" + $(if ($IsWindows) { ' -Clink-arg=/Brepro' } else { '' })
    $saved = @{ RUSTFLAGS = $env:RUSTFLAGS; CARGO_INCREMENTAL = $env:CARGO_INCREMENTAL; CARGO_TARGET_DIR = $env:CARGO_TARGET_DIR }
    $env:RUSTFLAGS = $flags; $env:CARGO_INCREMENTAL = '0'; $env:CARGO_TARGET_DIR = $target
    Push-Location (Join-Path $Root 'src')
    try {
        & cargo build --release --locked --offline --package eoie-cli 2>&1 | Select-Object -Last 3 | ForEach-Object { Write-Host "  $_" }
        if ($LASTEXITCODE -ne 0) { throw "cold build failed in $Root" }
    } finally {
        Pop-Location
        foreach ($k in $saved.Keys) { if ($null -eq $saved[$k]) { Remove-Item "Env:$k" -ErrorAction Ignore } else { Set-Item "Env:$k" $saved[$k] } }
    }
    Join-Path $target "release/eoie$suffix"
}

Write-Host "cold renewal: staging two release roots under $work"
$rootA = New-ReleaseRoot 'a'
$rootB = New-ReleaseRoot 'b'
$sourceA = Get-TreeSha $rootA
$sourceB = Get-TreeSha $rootB
Write-Host "cold renewal: source a=$sourceA b=$sourceB"

Write-Host 'cold renewal: cold build a'
$productA = Build-ColdProduct $rootA
Write-Host 'cold renewal: cold build b'
$productB = Build-ColdProduct $rootB
$productShaA = Get-Sha256 $productA
$productShaB = Get-Sha256 $productB
Write-Host "cold renewal: product a=$productShaA b=$productShaB"
if ($productShaA -cne $productShaB) { throw "cold rebuild products differ: a=$productShaA b=$productShaB" }

# Owners: plan the cold matrix in root a and replay it through batch-plan (one bounded compiler process per owner).
. $PSScriptRoot/spiral-compiler.ps1
$compiler = Get-SpiralCompilerDll 'single-flight'
$dotnet = Resolve-SpiralDotnet
$env:EOIE_SPIRAL_BUNDLE = $compilerRoot
$owners = @(Get-EoieOwners -Root $rootA -IncludeAuxiliary)
$rustfmt = (Get-Command rustfmt).Source
Copy-Item -LiteralPath $productA -Destination (Join-Path $rootA "eoie$suffix")
$matrixRelative = 'cold-matrix.spi'
Invoke-Eoie @('proxy', 'cold-rebuild-matrix', $rootA, $matrixRelative, $supervisor, $compiler, $dotnet, $rustfmt, "$OwnerTimeoutMs") | ForEach-Object { Write-Host "  $_" }
$reportRelative = 'cold-matrix-report.tsv'
$batchStart = Get-Date
Invoke-Eoie @('proxy', 'batch-plan', $rootA, $matrixRelative, "$($OwnerTimeoutMs + 60000)", $reportRelative, "$ParallelChains") | Select-Object -Last 3 | ForEach-Object { Write-Host "  $_" }
Write-Host "cold renewal: owner replay $([int]((Get-Date) - $batchStart).TotalSeconds) s at $ParallelChains parallel chains; report $(Join-Path $rootA $reportRelative)"
# The report is a TSV (id, program, rc, status, ..., elapsed_ms, ...): one row per planned owner command.
$rows = @(Import-Csv -LiteralPath (Join-Path $rootA $reportRelative) -Delimiter "`t")
$failed = @($rows | Where-Object { $_.rc -ne '0' })
if ($failed.Count) { throw "owner replay failed: $(($failed | ForEach-Object { "$($_.id) rc=$($_.rc)" }) -join ', ')" }
if ($rows.Count -ne $owners.Count) { Write-Host "cold renewal: note: replay rows=$($rows.Count), Get-EoieOwners=$($owners.Count)" }
$owners = $rows
$maxOwnerMs = ($rows | ForEach-Object { [int64]$_.elapsed_ms } | Measure-Object -Maximum).Maximum

# Global gate: the strict release gate on root b with product a (root b's sources are untouched by the replay).
Copy-Item -LiteralPath $productA -Destination (Join-Path $rootB "eoie$suffix")
$gateWatch = [Diagnostics.Stopwatch]::StartNew()
$gateOutput = & (Join-Path $rootB "eoie$suffix") bundle check $rootB eoie 2>&1 | ForEach-Object { "$_" }
$gateMs = $gateWatch.ElapsedMilliseconds
Write-Host "cold renewal: global gate $gateMs ms (exit $LASTEXITCODE; it checks the receipt this run replaces, so a cold rebuild error here is expected)"
$gateOutput | Select-Object -Last 3 | ForEach-Object { Write-Host "  $_" }

# Toolchain identity: the tools that built the products and replayed the owners.
function Get-ToolRow([string]$Path) { [pscustomobject]@{ Name = [IO.Path]::GetFileName($Path); Sha = (Get-Sha256 $Path); Bytes = (Get-Item -LiteralPath $Path).Length } }
$rustc = Get-ToolRow (& rustup which rustc); $cargo = Get-ToolRow (& rustup which cargo)
$rustdoc = Get-ToolRow (& rustup which rustdoc); $fmt = Get-ToolRow (& rustup which rustfmt)
$entry = Get-ToolRow $compiler; $facade = Get-ToolRow $dotnet
$identityPath = Join-Path $EoieRoot 'state/toolchain_identity.spi'
$identity = [IO.File]::ReadAllText($identityPath)
function Set-Identity([string]$Text, [string]$Function, $Row) {
    $pattern = "(?m)^(inl $Function \(\) : \w+ =\r?\n\s+\w+ \(\w+, EvidenceRef \()`"[^`"]*`", `"[0-9a-f]{64}`", \d+u64"
    if (-not [regex]::IsMatch($Text, $pattern)) { throw "toolchain identity row not found: $Function" }
    [regex]::Replace($Text, $pattern, "`${1}`"$($Row.Name)`", `"$($Row.Sha)`", $($Row.Bytes)u64")
}
foreach ($pair in @(@('rustc_identity', $rustc), @('cargo_identity', $cargo), @('rustdoc_identity', $rustdoc), @('rustfmt_identity', $fmt), @('spiral_entry_identity', $entry), @('spiral_facade_identity', $facade))) {
    $identity = Set-Identity $identity $pair[0] $pair[1]
}

$shardWidth = 2
$shards = [int][Math]::Ceiling($owners.Count / $shardWidth)
$normalization = "RUSTFLAGS=--remap-path-prefix=<cold-root>=/workspace -Cstrip=symbols" + $(if ($IsWindows) { ' -Clink-arg=/Brepro' } else { '' }) + ';CARGO_INCREMENTAL=0'
function Get-GateSha([string]$Relative, [string]$Override) {
    if ($Override) { return (Get-Sha256 $Override) }
    Get-Sha256 (Join-Path $EoieRoot $Relative)
}
$identityTemp = Join-Path $work 'toolchain_identity.spi'
[IO.File]::WriteAllText($identityTemp, $identity, [Text.UTF8Encoding]::new($false))
$receipt = @(
    'union cold_rebuild_schema ='
    '    | ColdRebuildV1'
    ''
    'union cold_rebuild_status ='
    '    | ColdRebuildClosed'
    ''
    'inl current_cold_rebuild_schema () : cold_rebuild_schema = ColdRebuildV1'
    'inl current_cold_rebuild_status () : cold_rebuild_status = ColdRebuildClosed'
    "inl current_owner_count () : u32 = $($owners.Count)u32"
    "inl current_shard_width () : u32 = $($shardWidth)u32"
    "inl current_shard_count () : u32 = $($shards)u32"
    'inl current_gate_flags () : u32 = 63u32'
    "inl current_max_owner_compile_ms () : u32 = $($maxOwnerMs)u32"
    "inl current_max_global_gate_ms () : u32 = $($gateMs)u32"
    ''
    "inl current_source_a_sha256 () : string = `"$sourceA`""
    "inl current_source_b_sha256 () : string = `"$sourceB`""
    "inl current_product_a_sha256 () : string = `"$productShaA`""
    "inl current_product_b_sha256 () : string = `"$productShaB`""
    "inl current_rustc_sha256 () : string = `"$($rustc.Sha)`""
    "inl current_compiler_entry_sha256 () : string = `"$($entry.Sha)`""
    "inl current_compiler_facade_sha256 () : string = `"$($facade.Sha)`""
    "inl current_rust_path_normalization () : string = `"$normalization`""
    ''
    "inl contract_gate () : string * string = `"src/cold_rebuild_contract/model.spi`", `"$(Get-GateSha 'src/cold_rebuild_contract/model.spi')`""
    "inl bindings_gate () : string * string = `"src/cold_rebuild_contract/bindings.spi`", `"$(Get-GateSha 'src/cold_rebuild_contract/bindings.spi')`""
    "inl matrix_gate () : string * string = `"src/eoie_cold_rebuild_matrix/main.spi`", `"$(Get-GateSha 'src/eoie_cold_rebuild_matrix/main.spi')`""
    "inl toolchain_gate () : string * string = `"state/toolchain_identity.spi`", `"$(Get-GateSha '' $identityTemp)`""
    ''
    'inl main () : i32 = 0i32'
) -join "`n"
$receiptTemp = Join-Path $work 'cold_rebuild.spi'
[IO.File]::WriteAllText($receiptTemp, $receipt + "`n", [Text.UTF8Encoding]::new($false))
Write-Host "cold renewal: owners=$($owners.Count) shards=$shards max_owner_ms=$maxOwnerMs global_gate_ms=$gateMs"
Write-Host "cold renewal: receipt $receiptTemp, toolchain identity $identityTemp, product $productA"
if ($maxOwnerMs -gt 15000 -or $gateMs -gt 30000) { throw "cold rebuild budget exceeded: owner_ms=$maxOwnerMs global_ms=$gateMs" }
if ($Apply) {
    [IO.File]::Copy($identityTemp, $identityPath, $true)
    [IO.File]::Copy($receiptTemp, (Join-Path $EoieRoot 'state/cold_rebuild.spi'), $true)
    $published = Publish-EoieBinary -Source $productA -Destination (Join-Path $EoieRoot "eoie$suffix") -ValidationRoot $EoieRoot
    Write-Host "cold renewal: applied (state/toolchain_identity.spi, state/cold_rebuild.spi); published=$published"
}
[pscustomobject]@{ Work = $work; Receipt = $receiptTemp; Identity = $identityTemp; Product = $productA; ProductSha = $productShaA; Owners = $owners.Count; MaxOwnerMs = $maxOwnerMs; GateMs = $gateMs }
