function Resolve-EoieSpiralBundle {
    param([string]$CompilerBundle)
    if (-not $CompilerBundle) { $CompilerBundle = $env:EOIE_SPIRAL_BUNDLE }
    if (-not $CompilerBundle) { $CompilerBundle = Join-Path $PSScriptRoot '../../spiral/apps/compiler/tmp' }
    if (-not (Test-Path -LiteralPath (Join-Path $CompilerBundle 'scripts/env.ps1'))) {
        throw "Spiral compiler bundle not found at $CompilerBundle. Clone https://github.com/i574n/spiral beside this repository or set EOIE_SPIRAL_BUNDLE to its apps/compiler/tmp."
    }
    (Resolve-Path -LiteralPath $CompilerBundle).Path
}

function Copy-EoieCompilerSnapshot {
    param([Parameter(Mandatory)][string]$Compiler, [Parameter(Mandatory)][string]$Destination)
    if ([IO.Path]::GetExtension($Compiler) -ine '.dll') { return $Compiler }
    $source = Split-Path $Compiler -Parent
    $entries = @(Get-ChildItem -LiteralPath $source -Recurse -Force)
    if ($entries | Where-Object { $_.Attributes -band [IO.FileAttributes]::ReparsePoint }) {
        throw "Compiler snapshot rejects links: $source"
    }
    $snapshot = @($entries | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
        [pscustomobject]@{
            Relative = [IO.Path]::GetRelativePath($source, $_.FullName)
            Source = $_.FullName
            Hash = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash
        }
    })
    foreach ($file in $snapshot) {
        $copy = Join-Path $Destination $file.Relative
        [void][IO.Directory]::CreateDirectory((Split-Path $copy -Parent))
        Copy-Item -LiteralPath $file.Source -Destination $copy
        if ((Get-FileHash -LiteralPath $copy -Algorithm SHA256).Hash -cne $file.Hash) {
            throw "Compiler changed while taking snapshot: $($file.Source)"
        }
    }
    foreach ($file in $snapshot) {
        if ((Get-FileHash -LiteralPath $file.Source -Algorithm SHA256).Hash -cne $file.Hash) {
            throw "Compiler changed while taking snapshot: $($file.Source)"
        }
    }
    if (@(Get-ChildItem -LiteralPath $source -Recurse -Force -File).Count -ne $snapshot.Count) {
        throw "Compiler files changed while taking snapshot: $source"
    }
    $manifest = $snapshot | Select-Object Relative, Hash | ConvertTo-Json
    [IO.File]::WriteAllText((Join-Path $Destination 'eoie-compiler-snapshot.json'), $manifest, [Text.UTF8Encoding]::new($false))
    return Join-Path $Destination ([IO.Path]::GetFileName($Compiler))
}

function Get-EoieCompilerSidecarGuards {
    param([Parameter(Mandatory)][object[]]$Rows)
    $comparer = if ($IsWindows) { [StringComparer]::OrdinalIgnoreCase } else { [StringComparer]::Ordinal }
    $targets = [Collections.Generic.HashSet[string]]::new($comparer)
    $seen = [Collections.Generic.HashSet[string]]::new($comparer)
    foreach ($row in $Rows) { [void]$targets.Add($row.Target) }
    foreach ($row in $Rows) {
        $path = [IO.Path]::ChangeExtension($row.Input, '.rs')
        if ($targets.Contains($path) -or -not $seen.Add($path)) { continue }
        $existed = [IO.File]::Exists($path)
        $content = $null
        if ($existed) { $content = [IO.File]::ReadAllBytes($path) }
        [pscustomobject]@{ Path = $path; Existed = $existed; Content = $content; Output = $row.Output }
    }
}

function Restore-EoieCompilerSidecars {
    param([Parameter(Mandatory)][string]$Root, [object[]]$Guards = @())
    $prefix = [IO.Path]::GetFullPath($Root).TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    $comparison = if ($IsWindows) { [StringComparison]::OrdinalIgnoreCase } else { [StringComparison]::Ordinal }
    $removed = 0
    $restored = 0
    foreach ($guard in $Guards) {
        if (-not [IO.Path]::GetFullPath($guard.Path).StartsWith($prefix, $comparison)) { throw "Compiler sidecar escapes staged root: $($guard.Path)" }
        if ($guard.Existed) {
            [IO.File]::WriteAllBytes($guard.Path, [byte[]]$guard.Content)
            $restored++
        } elseif ([IO.File]::Exists($guard.Path)) {
            if ((Get-FileHash -LiteralPath $guard.Path -Algorithm SHA256).Hash -cne (Get-FileHash -LiteralPath $guard.Output -Algorithm SHA256).Hash) {
                throw "Unexpected compiler sidecar content: $($guard.Path)"
            }
            [IO.File]::Delete($guard.Path)
            $removed++
        }
    }
    Write-Host "Removed $removed new compiler sidecars; restored $restored copied adapters."
}

function Set-EoieGeneratedFile {
    param([Parameter(Mandatory)][string]$Source, [Parameter(Mandatory)][string]$Destination)
    $content = [IO.File]::ReadAllText($Source).Replace("`r`n", "`n").TrimEnd() + "`n"
    if ([IO.File]::Exists($Destination) -and [IO.File]::ReadAllText($Destination) -ceq $content) {
        return $false
    }
    $parent = Split-Path $Destination -Parent
    [void][IO.Directory]::CreateDirectory($parent)
    $temporary = Join-Path $parent ('.' + [IO.Path]::GetFileName($Destination) + '.' + [guid]::NewGuid().ToString('N') + '.tmp')
    try {
        [IO.File]::WriteAllText($temporary, $content, [Text.UTF8Encoding]::new($false))
        [IO.File]::Move($temporary, $Destination, $true)
    } finally {
        if ([IO.File]::Exists($temporary)) { [IO.File]::Delete($temporary) }
    }
    return $true
}

function Publish-EoieBinary {
    param(
        [Parameter(Mandatory)][string]$Source,
        [Parameter(Mandatory)][string]$Destination,
        [ValidateRange(50, 60000)][int]$TimeoutMs = 30000,
        [string]$ValidationRoot
    )
    $Source = (Resolve-Path -LiteralPath $Source).Path
    $Destination = [IO.Path]::GetFullPath($Destination, $PWD.Path)
    $checks = @(@{ Name = 'schema'; Arguments = @('help', '--schema') })
    if ($ValidationRoot) {
        $ValidationRoot = (Resolve-Path -LiteralPath $ValidationRoot).Path
        $checks += @{ Name = 'topology'; Arguments = @('proxy', 'source-topology', $ValidationRoot) }
    }
    $parent = Split-Path $Destination -Parent
    [void][IO.Directory]::CreateDirectory($parent)
    $priorHash = if ([IO.File]::Exists($Destination)) { (Get-FileHash -LiteralPath $Destination -Algorithm SHA256).Hash } else { $null }
    $sourceHash = (Get-FileHash -LiteralPath $Source -Algorithm SHA256).Hash
    $temporary = Join-Path $parent ('.release-eoie-candidate-' + [guid]::NewGuid().ToString('N') + $(if ($IsWindows) { '.exe' } else { '' }))
    try {
        [IO.File]::Copy($Source, $temporary, $false)
        if ((Get-FileHash -LiteralPath $temporary -Algorithm SHA256).Hash -cne $sourceHash) { throw 'Release executable changed during staging.' }
        if (-not $IsWindows) { [IO.File]::SetUnixFileMode($temporary, [IO.File]::GetUnixFileMode($Source)) }
        foreach ($check in $checks) {
            $start = [Diagnostics.ProcessStartInfo]::new($temporary)
            $start.UseShellExecute = $false
            $start.CreateNoWindow = $true
            $start.WorkingDirectory = $parent
            $start.RedirectStandardOutput = $true
            $start.RedirectStandardError = $true
            foreach ($argument in $check.Arguments) { $start.ArgumentList.Add($argument) }
            $process = [Diagnostics.Process]::new()
            $process.StartInfo = $start
            try {
                [void]$process.Start()
                $stdout = $process.StandardOutput.ReadToEndAsync()
                $stderr = $process.StandardError.ReadToEndAsync()
                if (-not $process.WaitForExit($TimeoutMs)) {
                    $process.Kill($true)
                    if (-not $process.WaitForExit(5000)) { throw 'EOIE candidate did not exit after termination.' }
                    throw "EOIE candidate $($check.Name) check timed out after $TimeoutMs ms."
                }
                if (-not [Threading.Tasks.Task]::WaitAll([Threading.Tasks.Task[]]@($stdout, $stderr), $TimeoutMs)) {
                    throw 'EOIE candidate output streams did not close within the smoke budget.'
                }
                $output = $stdout.GetAwaiter().GetResult()
                $errorText = $stderr.GetAwaiter().GetResult()
                $validOutput = if ($check.Name -eq 'schema') {
                    ($output.Trim() -split '\r?\n')[0] -ceq 'command-spec'
                } else { $output -cmatch '(?m)^topology_limits_ok=true(?: |\r?$)' }
                if ($process.ExitCode -ne 0 -or -not $validOutput) {
                    throw "EOIE candidate $($check.Name) check failed (exit $($process.ExitCode)): $errorText"
                }
            } finally { $process.Dispose() }
        }
        if ((Get-FileHash -LiteralPath $temporary -Algorithm SHA256).Hash -cne $sourceHash) { throw 'Release candidate changed during validation.' }
        $currentHash = if ([IO.File]::Exists($Destination)) { (Get-FileHash -LiteralPath $Destination -Algorithm SHA256).Hash } else { $null }
        if ($currentHash -cne $priorHash) { throw 'Installed EOIE changed during candidate validation.' }
        if ($currentHash -ceq $sourceHash) { return $false }
        [IO.File]::Move($temporary, $Destination, $true)
        return $true
    } finally {
        if ([IO.File]::Exists($temporary)) { [IO.File]::Delete($temporary) }
    }
}

function Get-EoieOwners {
    param([Parameter(Mandatory)][string]$Root, [string[]]$Package = @(), [string]$Filter = '.*', [switch]$IncludeAuxiliary)
    $sourceRoot = (Resolve-Path -LiteralPath (Join-Path $Root 'src')).Path
    Push-Location $sourceRoot
    try {
        $metadataText = & cargo metadata --no-deps --format-version 1 --locked --offline
        if ($LASTEXITCODE -ne 0) { throw 'Cannot read EOIE Cargo workspace metadata.' }
        $metadata = ($metadataText -join "`n") | ConvertFrom-Json -AsHashtable
    } finally {
        Pop-Location
    }
    $available = @($metadata.packages | Where-Object { $_.id -in $metadata.workspace_members })
    foreach ($name in $Package) {
        if ($name -cnotin $available.name) { throw "Unknown EOIE package '$name'. Use dev.ps1 -List to see available names." }
    }
    foreach ($item in $available) {
        if ($Package.Count -and $item.name -cnotin $Package) { continue }
        $directory = Split-Path $item.manifest_path -Parent
        $member = [IO.Path]::GetRelativePath($sourceRoot, $directory).Replace('\', '/')
        if ($member -notmatch $Filter) { continue }
        if (-not $item.metadata -or -not $item.metadata.spiral -or -not $item.metadata.spiral.entry -or -not $item.metadata.spiral.output) {
            throw "Package '$($item.name)' needs [package.metadata.spiral] entry and output in Cargo.toml."
        }
        $declarations = @($item.metadata.spiral)
        if ($IncludeAuxiliary -and $item.metadata.spiral.ContainsKey('auxiliary')) {
            if ($item.metadata.spiral.auxiliary -isnot [array]) { throw "Spiral auxiliary owners must be an array: $($item.name)" }
            $declarations += $item.metadata.spiral.auxiliary
        }
        $seenOutputs = [Collections.Generic.HashSet[string]]::new($(if ($IsWindows) { [StringComparer]::OrdinalIgnoreCase } else { [StringComparer]::Ordinal }))
        foreach ($declaration in $declarations) {
            if ($declaration -isnot [Collections.IDictionary] -or -not $declaration.entry -or -not $declaration.output) {
                throw "Spiral declaration needs entry and output: $($item.name)"
            }
            $entry = [IO.Path]::GetFullPath($declaration.entry, $directory)
            $relativeEntry = [IO.Path]::GetRelativePath($directory, $entry)
            if ([IO.Path]::IsPathRooted($relativeEntry) -or $relativeEntry -eq '..' -or $relativeEntry.StartsWith('../') -or $relativeEntry.StartsWith('..\')) {
                throw "Spiral entry escapes package '$($item.name)': $entry"
            }
            if (-not (Test-Path -LiteralPath $entry -PathType Leaf)) { throw "Missing Spiral entry: $entry" }
            $output = [IO.Path]::GetFullPath($declaration.output, $directory)
            $relativeOutput = [IO.Path]::GetRelativePath($directory, $output)
            if ([IO.Path]::IsPathRooted($relativeOutput) -or $relativeOutput -eq '..' -or $relativeOutput.StartsWith('../') -or $relativeOutput.StartsWith('..\')) {
                throw "Spiral output escapes package '$($item.name)': $output"
            }
            $targets = @($item.targets | Where-Object { [IO.Path]::GetFullPath($_.src_path) -ceq $output })
            if ($targets.Count -ne 1) { throw "Spiral output must identify one Cargo target for '$($item.name)': $output" }
            if (-not $seenOutputs.Add($output)) { throw "Duplicate Spiral output: $output" }
            $id = if ($seenOutputs.Count -eq 1) { $member } else { $member + '__' + ($targets[0].kind -join '-') + '_' + $targets[0].name }
            [pscustomobject]@{ Member = $member; Id = $id; Package = $item.name; Input = $entry; Target = $targets[0].src_path }
        }
    }
}
