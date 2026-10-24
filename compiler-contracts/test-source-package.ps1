[CmdletBinding()]
param(
    [string]$EoieRoot = (Join-Path $PSScriptRoot '..'),
    [string]$EoieBinary,
    [string]$TargetDirectory,
    [switch]$Offline,
    [switch]$SkipBuild,
    [switch]$PassThru
)
$ErrorActionPreference = 'Stop'
$EoieRoot = (Resolve-Path -LiteralPath $EoieRoot).Path
if (-not $EoieBinary) { $EoieBinary = Join-Path $EoieRoot $(if ($IsWindows) { 'eoie.exe' } else { 'eoie' }) }
$EoieBinary = (Resolve-Path -LiteralPath $EoieBinary).Path
if (-not $TargetDirectory) { $TargetDirectory = Join-Path $EoieRoot 'src/target/source-package-validation' }
$TargetDirectory = [IO.Path]::GetFullPath($TargetDirectory, $PWD.Path)
$work = Join-Path $EoieRoot ('.cache/source-package/' + [guid]::NewGuid().ToString('N'))
$staged = Join-Path $work 'sources'
$restored = Join-Path $work 'rehydrated sources ü'
$archive = Join-Path $work 'eoie-source.zip'
[void][IO.Directory]::CreateDirectory($staged)

$start = [Diagnostics.ProcessStartInfo]::new('git')
$start.UseShellExecute = $false
$start.CreateNoWindow = $true
$start.RedirectStandardOutput = $true
$start.RedirectStandardError = $true
$start.StandardOutputEncoding = [Text.UTF8Encoding]::new($false)
foreach ($argument in @('-C', $EoieRoot, 'ls-files', '--cached', '--others', '--exclude-standard', '-z', '--', '.')) { $start.ArgumentList.Add($argument) }
$process = [Diagnostics.Process]::new()
$process.StartInfo = $start
try {
    [void]$process.Start()
    $stdout = $process.StandardOutput.ReadToEndAsync()
    $stderr = $process.StandardError.ReadToEndAsync()
    if (-not $process.WaitForExit(30000)) { $process.Kill($true); throw 'Git source inventory timed out.' }
    $output = $stdout.GetAwaiter().GetResult()
    $errorText = $stderr.GetAwaiter().GetResult()
    if ($process.ExitCode -ne 0) { throw "Git source inventory failed: $errorText" }
} finally { $process.Dispose() }
$paths = @($output.Split([char]0, [StringSplitOptions]::RemoveEmptyEntries) | Sort-Object -CaseSensitive -Unique)
if (-not $paths.Count) { throw 'Git source inventory is empty.' }
$comparison = if ($IsWindows) { [StringComparison]::OrdinalIgnoreCase } else { [StringComparison]::Ordinal }
$prefix = $EoieRoot.TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
$manifest = [Collections.Generic.List[object]]::new()
foreach ($relative in $paths) {
    $source = [IO.Path]::GetFullPath($relative, $EoieRoot)
    if (-not $source.StartsWith($prefix, $comparison)) { throw "Git source path escapes EOIE: $relative" }
    if (-not (Test-Path -LiteralPath $source)) { continue }
    if ($relative -match '(^|/)(vendor|target|\.cache|\.git)(/|$)' -or $relative -match '^(eoie|eoie\.exe)$') {
        throw "Build artifact is Git-eligible: $relative"
    }
    $cursor = $source
    while ($cursor.Length -ge $EoieRoot.Length) {
        $entry = Get-Item -LiteralPath $cursor -Force
        if ($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Source package rejects links: $cursor" }
        if ($cursor -eq $EoieRoot) { break }
        $cursor = Split-Path $cursor -Parent
    }
    if (-not (Test-Path -LiteralPath $source -PathType Leaf)) { throw "Git source is not a regular file: $relative" }
    $destination = Join-Path $staged $relative
    [void][IO.Directory]::CreateDirectory((Split-Path $destination -Parent))
    $hash = (Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash
    Copy-Item -LiteralPath $source -Destination $destination
    if ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash -cne $hash) { throw "Source changed during copy: $relative" }
    $manifest.Add([pscustomobject]@{ Path = $relative; Bytes = (Get-Item -LiteralPath $destination -Force).Length; SHA256 = $hash })
}
foreach ($file in $manifest) {
    if ((Get-FileHash -LiteralPath (Join-Path $EoieRoot $file.Path) -Algorithm SHA256).Hash -cne $file.SHA256) { throw "Source changed during snapshot: $($file.Path)" }
}
[IO.File]::WriteAllText((Join-Path $work 'manifest.json'), ($manifest | ConvertTo-Json), [Text.UTF8Encoding]::new($false))
& $EoieBinary bundle create-flat $staged $archive 30000000
if ($LASTEXITCODE -ne 0) { throw 'EOIE source archive creation failed.' }
& $EoieBinary bundle verify-flat $archive 30000000
if ($LASTEXITCODE -ne 0) { throw 'EOIE source archive verification failed.' }
& $EoieBinary bundle rehydrate $archive $restored generic
if ($LASTEXITCODE -ne 0) { throw 'EOIE source rehydration failed.' }
$restoredFiles = @(Get-ChildItem -LiteralPath $restored -Recurse -File -Force)
if ($restoredFiles.Count -ne $manifest.Count) { throw 'Rehydrated source file count differs.' }
foreach ($file in $manifest) {
    if ((Get-FileHash -LiteralPath (Join-Path $restored $file.Path) -Algorithm SHA256).Hash -cne $file.SHA256) { throw "Rehydrated content differs: $($file.Path)" }
}
& $EoieBinary proxy source-topology $restored
if ($LASTEXITCODE -ne 0) { throw 'Rehydrated source topology failed.' }
if (-not $SkipBuild) {
    $previousJobs = $env:CARGO_BUILD_JOBS
    $previousThreads = $env:RUST_TEST_THREADS
    try {
        $env:CARGO_BUILD_JOBS = '2'
        $env:RUST_TEST_THREADS = '1'
        & (Join-Path $restored 'build.ps1') -Test -SkipRelease -TargetDirectory $TargetDirectory -Offline:$Offline
        if ($LASTEXITCODE -ne 0) { throw 'Rehydrated source build or native contracts failed.' }
    } finally {
        $env:CARGO_BUILD_JOBS = $previousJobs
        $env:RUST_TEST_THREADS = $previousThreads
    }
}
Write-Host "EOIE source package passed: files=$($manifest.Count) native_build=$(-not $SkipBuild) manifest=$(Join-Path $work 'manifest.json')"
if ($PassThru) {
    [pscustomobject]@{
        Manifest = Join-Path $work 'manifest.json'
        RestoredRoot = $restored
        Archive = $archive
        FileCount = $manifest.Count
        NativeBuild = -not $SkipBuild
    }
}
