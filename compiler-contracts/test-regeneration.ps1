[CmdletBinding()]
param(
    [string]$EoieRoot,
    [string]$Filter = '.*',
    [string[]]$Package = @(),
    [switch]$IncludeAuxiliary,
    [string]$TargetDirectory,
    [string]$CompilerBundle = $env:EOIE_SPIRAL_BUNDLE,
    [ValidateRange(1, 600)][int]$TimeoutSec = 180,
    [switch]$CargoCheck,
    [switch]$Test,
    [switch]$SkipRelease,
    [switch]$Publish,
    [switch]$CompilerContracts,
    [switch]$Offline
)
$ErrorActionPreference = 'Stop'
if ($TargetDirectory) { $TargetDirectory = [IO.Path]::GetFullPath($TargetDirectory, $PWD.Path) }
if ($CompilerContracts -and -not $Test) { throw '-CompilerContracts requires -Test.' }
if ($Publish -and -not ($CargoCheck -or $Test)) { throw '-Publish requires -CargoCheck or -Test.' }
. $PSScriptRoot/workspace.ps1
if (-not $EoieRoot) { $EoieRoot = Join-Path $PSScriptRoot '..' }
$EoieRoot = (Resolve-Path -LiteralPath $EoieRoot).Path
$selected = @(Get-EoieOwners -Root $EoieRoot -Package $Package -Filter $Filter -IncludeAuxiliary:$IncludeAuxiliary)
if (-not $selected.Count) { throw 'No regeneration owners selected.' }
$CompilerBundle = Resolve-EoieSpiralBundle $CompilerBundle
. (Join-Path $CompilerBundle 'scripts/env.ps1')
$dotnet = Resolve-SpiralDotnet
$compiler = Get-SpiralCompilerDll 'single-flight'
$work = Join-Path (Get-SpiralCacheDir) ('eoie-regeneration/' + [guid]::NewGuid().ToString('N'))
$compiler = Copy-EoieCompilerSnapshot -Compiler $compiler -Destination (Join-Path $work 'compiler')
$staged = Join-Path $work 'eoie'
$snapshot = [Collections.Generic.List[object]]::new()
function Get-WorkspaceEntries([string]$Directory) {
    foreach ($entry in Get-ChildItem -LiteralPath $Directory -Force) {
        if ($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Unexpected workspace link: $($entry.FullName)" }
        if ($entry.PSIsContainer) {
            if ($entry.Name -notin @('target', 'vendor', '.git', '.cache')) { $entry }
        } elseif ($entry.Name -notin @('eoie', 'eoie.exe')) { $entry }
    }
}
function Get-WorkspaceFiles([string]$Directory) {
    foreach ($entry in Get-WorkspaceEntries $Directory) {
        if ($entry.PSIsContainer) { Get-WorkspaceFiles $entry.FullName }
        else { $entry.FullName }
    }
}
function Copy-Workspace([string]$From, [string]$To) {
    New-Item -ItemType Directory -Path $To -Force | Out-Null
    foreach ($entry in Get-WorkspaceEntries $From) {
        if ($entry.PSIsContainer) {
            Copy-Workspace $entry.FullName (Join-Path $To $entry.Name)
        } else {
            $copy = Join-Path $To $entry.Name
            Copy-Item -LiteralPath $entry.FullName -Destination $copy
            if ($Publish) {
                $snapshot.Add([pscustomobject]@{ Path = $entry.FullName; Hash = (Get-FileHash -LiteralPath $copy -Algorithm SHA256).Hash })
            }
        }
    }
}
Copy-Workspace $EoieRoot $staged
$sourceRoot = Join-Path $staged 'src'
if (-not $TargetDirectory) { $TargetDirectory = Join-Path $sourceRoot 'target' }
$rows = @()
foreach ($owner in $selected) {
    $entry = Join-Path $staged ([IO.Path]::GetRelativePath($EoieRoot, $owner.Input))
    $target = Join-Path $staged ([IO.Path]::GetRelativePath($EoieRoot, $owner.Target))
    $output = Join-Path $work "outputs/$($owner.Id).rs"
    $rows += [pscustomobject]@{ Member = $owner.Id; Package = $owner.Package; Input = $entry; Output = $output; Target = $target; Original = $owner.Target }
}
$sidecars = @(Get-EoieCompilerSidecarGuards -Rows $rows)
New-Item -ItemType Directory -Path (Join-Path $work 'outputs') -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $work 'logs') -Force | Out-Null
$results = Join-Path $work 'results.tsv'
$rows | Export-Csv -LiteralPath (Join-Path $work 'owners.csv') -NoTypeInformation
$previousWorkspace = $env:SPIRAL_WORKSPACE_ROOT
$previousBudget = $env:SPIRAL_BUILD_BUDGET_MS
try {
    $env:SPIRAL_WORKSPACE_ROOT = $BundleRoot
    Write-Host "Regenerating $($rows.Count) declared EOIE outputs: $work"
    $encoding = [Text.UTF8Encoding]::new($false)
    $writer = [IO.StreamWriter]::new($results, $false, $encoding)
    try {
        $writer.AutoFlush = $true
        foreach ($row in $rows) {
            $timeoutMs = $TimeoutSec * 1000
            $stdout = Join-Path $work "logs/$($row.Member).out"
            $stderr = Join-Path $work "logs/$($row.Member).err"
            $env:SPIRAL_BUILD_BUDGET_MS = [string]([Math]::Max(1000, $timeoutMs - 3000))
            $watch = [Diagnostics.Stopwatch]::StartNew()
            $start = [Diagnostics.ProcessStartInfo]::new()
            $start.FileName = $dotnet
            $start.WorkingDirectory = $work
            $start.UseShellExecute = $false
            $start.CreateNoWindow = $true
            $start.RedirectStandardOutput = $true
            $start.RedirectStandardError = $true
            foreach ($argument in @($compiler, '--backend', 'Rust', $row.Input, $row.Output)) {
                $start.ArgumentList.Add($argument)
            }
            $proc = [Diagnostics.Process]::new()
            $proc.StartInfo = $start
            try {
                if (-not $proc.Start()) { throw "Cannot start compiler for $($row.Member)" }
                $outTask = $proc.StandardOutput.ReadToEndAsync()
                $errTask = $proc.StandardError.ReadToEndAsync()
                $timedOut = -not $proc.WaitForExit($timeoutMs)
                if ($timedOut) {
                    $proc.Kill($true)
                    if (-not $proc.WaitForExit(10000)) { throw "Compiler did not terminate: $($row.Member)" }
                }
                [IO.File]::WriteAllText($stdout, $outTask.GetAwaiter().GetResult())
                [IO.File]::WriteAllText($stderr, $errTask.GetAwaiter().GetResult())
                $exitCode = $proc.ExitCode
            } finally {
                $proc.Dispose()
            }
            if ($timedOut) {
                $writer.WriteLine("$($row.Member)`ttimeout`t$($watch.ElapsedMilliseconds)`tno result within $timeoutMs ms")
                Write-Host "$($row.Member) timeout $($watch.ElapsedMilliseconds) ms"
                continue
            }
            $elapsed = $watch.ElapsedMilliseconds
            if ($exitCode -eq 0 -and (Test-Path -LiteralPath $row.Output)) {
                $bytes = (Get-Item -LiteralPath $row.Output).Length
                $writer.WriteLine("$($row.Member)`tok`t$elapsed`tbytes=$bytes entry=main revision=process")
                Write-Host "$($row.Member) ok $elapsed ms ($bytes bytes)"
            } else {
                $detail = ''
                if (Test-Path -LiteralPath $stderr) {
                    $detail = ([IO.File]::ReadAllText($stderr) -replace '[\t\r\n]', ' ').Trim()
                }
                if (-not $detail -and (Test-Path -LiteralPath $stdout)) {
                    $detail = ([IO.File]::ReadAllText($stdout) -replace '[\t\r\n]', ' ').Trim()
                }
                if ($detail.Length -gt 500) { $detail = $detail.Substring(0, 500) }
                if (-not $detail) { $detail = "compiler exit $exitCode" }
                $writer.WriteLine("$($row.Member)`terror`t$elapsed`t$detail")
                Write-Host "$($row.Member) error $elapsed ms $detail"
            }
        }
    } finally {
        $writer.Dispose()
    }
} finally {
    $env:SPIRAL_WORKSPACE_ROOT = $previousWorkspace
    if ($null -eq $previousBudget) { Remove-Item Env:SPIRAL_BUILD_BUDGET_MS -ErrorAction SilentlyContinue }
    else { $env:SPIRAL_BUILD_BUDGET_MS = $previousBudget }
}
$compiled = @(Import-Csv -LiteralPath $results -Delimiter "`t" -Header 'owner','status','elapsed','detail')
$failed = @($compiled | Where-Object status -ne 'ok')
Write-Host "Emitted: $(@($compiled | Where-Object status -eq 'ok').Count)/$($rows.Count); results: $results"
if ($failed.Count -or $compiled.Count -ne $rows.Count) {
    $failed | Format-Table owner,status,detail -Wrap
    throw "EOIE regeneration incomplete; committed Rust was not replaced. Results: $results"
}
Restore-EoieCompilerSidecars -Root $staged -Guards $sidecars
foreach ($row in $rows) {
    if (-not (Test-Path -LiteralPath $row.Output)) { throw "Missing emitted owner: $($row.Member)" }
    Copy-Item -LiteralPath $row.Output -Destination $row.Target
}
if ($CargoCheck) {
    $cargoArgs = @('check', '--target-dir', $TargetDirectory, '--all-targets', '--locked')
    if ($Package.Count) {
        foreach ($name in $Package) { $cargoArgs += @('--package', $name) }
    } else { $cargoArgs += '--workspace' }
    if ($Offline) { $cargoArgs += '--offline' }
    Push-Location $sourceRoot
    try {
        & cargo @cargoArgs
        if ($LASTEXITCODE -ne 0) { throw "Cargo rejected regenerated EOIE owners: $staged" }
    } finally { Pop-Location }
}
if ($Test) {
    $previousDotnet = $env:EOIE_DOTNET
    $previousBundle = $env:EOIE_SPIRAL_BUNDLE
    try {
        $env:EOIE_DOTNET = $dotnet
        $env:EOIE_SPIRAL_BUNDLE = $BundleRoot
        & (Join-Path $staged 'build.ps1') -Test -Package $Package -TargetDirectory $TargetDirectory -SkipRelease:$SkipRelease -CompilerContracts:$CompilerContracts -Offline:$Offline -SpiralCompiler $compiler
        if ($LASTEXITCODE -ne 0) { throw "Regenerated EOIE runtime contracts failed: $staged" }
    } finally {
        $env:EOIE_DOTNET = $previousDotnet
        $env:EOIE_SPIRAL_BUNDLE = $previousBundle
    }
    if (-not $Package.Count) {
        $binaryName = if ($IsWindows) { 'eoie.exe' } else { 'eoie' }
        $validatedBinary = Join-Path $TargetDirectory "debug/$binaryName"
        & $validatedBinary proxy source-topology $staged
        if ($LASTEXITCODE -ne 0) { throw "Regenerated EOIE source topology failed: $staged" }
    }
}
if ($Publish) {
    $currentFiles = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    foreach ($path in Get-WorkspaceFiles $EoieRoot) { [void]$currentFiles.Add($path) }
    if ($currentFiles.Count -ne $snapshot.Count -or @($snapshot | Where-Object { -not $currentFiles.Contains($_.Path) }).Count) {
        throw 'Checkout changed during validation; source file inventory differs and generated outputs were not published.'
    }
    foreach ($file in $snapshot) {
        if (-not (Test-Path -LiteralPath $file.Path -PathType Leaf) -or (Get-FileHash -LiteralPath $file.Path -Algorithm SHA256).Hash -cne $file.Hash) {
            throw "Checkout changed during validation; generated outputs were not published: $($file.Path)"
        }
    }
    $changed = 0
    foreach ($row in $rows) {
        if (Set-EoieGeneratedFile -Source $row.Output -Destination $row.Original) { $changed++ }
    }
    Write-Host "Published $changed changed owners; $($rows.Count - $changed) unchanged."
}
Write-Host "EOIE regeneration passed for $($rows.Count) selected owners. Staged workspace: $staged"
