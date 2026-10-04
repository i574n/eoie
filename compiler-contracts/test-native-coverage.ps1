[CmdletBinding()]
param(
    [string]$EoieRoot = (Join-Path $PSScriptRoot '..'),
    [string]$EoieBinary,
    [Parameter(Mandatory)][string]$SpiralCompiler,
    [Parameter(Mandatory)][string]$LlvmDirectory,
    [string]$TargetDirectory,
    [ValidateRange(1, 256)][int]$Jobs = 2,
    [ValidateRange(1000, 3600000)][int]$TimeoutMs = 1200000,
    [switch]$CompilerContracts
)
$ErrorActionPreference = 'Stop'
$EoieRoot = (Resolve-Path -LiteralPath $EoieRoot).Path
if (-not $EoieBinary) { $EoieBinary = Join-Path $EoieRoot $(if ($IsWindows) { 'eoie.exe' } else { 'eoie' }) }
$EoieBinary = (Resolve-Path -LiteralPath $EoieBinary).Path
$SpiralCompiler = (Resolve-Path -LiteralPath $SpiralCompiler).Path
$LlvmDirectory = (Resolve-Path -LiteralPath $LlvmDirectory).Path
$suffix = if ($IsWindows) { '.exe' } else { '' }
$profdata = (Resolve-Path -LiteralPath (Join-Path $LlvmDirectory "llvm-profdata$suffix")).Path
$llvmCov = (Resolve-Path -LiteralPath (Join-Path $LlvmDirectory "llvm-cov$suffix")).Path
$cargo = (Get-Command cargo -CommandType Application | Select-Object -First 1).Source
$rustc = (Get-Command rustc -CommandType Application | Select-Object -First 1).Source
if (-not $TargetDirectory) { $TargetDirectory = Join-Path $EoieRoot 'src/target/native-coverage' }
$TargetDirectory = [IO.Path]::GetFullPath($TargetDirectory, $PWD.Path)
$work = Join-Path $EoieRoot ('.cache/native-coverage/' + [guid]::NewGuid().ToString('N'))
[void][IO.Directory]::CreateDirectory($work)
$encoding = [Text.UTF8Encoding]::new($false)
Copy-Item -LiteralPath $EoieBinary -Destination (Join-Path $work "eoie$suffix")
$EoieBinary = Join-Path $work "eoie$suffix"

function Invoke-Captured([string]$Name, [string]$Program, [string[]]$Arguments, [switch]$RejectStderr, [switch]$Quiet) {
    $receiptName = "$Name.txt"
    & $EoieBinary proxy command-capture $work $receiptName $TimeoutMs $Program @Arguments | ForEach-Object { if (-not $Quiet) { Write-Host $_ } }
    if ($LASTEXITCODE -ne 0) { throw "EOIE could not capture $Name." }
    $receipt = [IO.File]::ReadAllText((Join-Path $work $receiptName))
    $marker = "--- stdout ---" + [char]10
    $start = $receipt.IndexOf($marker, [StringComparison]::Ordinal)
    $end = $receipt.LastIndexOf(([char]10 + "--- stderr ---" + [char]10), [StringComparison]::Ordinal)
    if ($start -lt 0 -or $end -lt $start) { throw "Invalid $Name receipt framing." }
    $header = $receipt.Substring(0, $start)
    if ($header -notmatch '(?m)^status=0\r?$' -or $header -notmatch '(?m)^termination=Exited\(0\)\r?$') {
        throw "$Name failed; inspect $(Join-Path $work $receiptName)"
    }
    if ($RejectStderr -and $header -notmatch '(?m)^stderr_bytes=0\r?$') { throw "$Name emitted diagnostics; inspect $(Join-Path $work $receiptName)" }
    $payload = $receipt.Substring($start + $marker.Length, $end - $start - $marker.Length)
    $byteMatch = [regex]::Match($header, '(?m)^stdout_bytes=(\d+)\r?$')
    if (-not $byteMatch.Success -or $encoding.GetByteCount($payload) -ne [long]$byteMatch.Groups[1].Value) { throw "$Name receipt payload length differs." }
    return $payload
}

function Write-LlvmArguments([string]$Path, [string[]]$Arguments) {
    # LLVM response files avoid the Windows command-line length limit.
    $quoted = foreach ($argument in $Arguments) {
        if ($argument.Contains([char]10) -or $argument.Contains([char]13)) { throw 'LLVM arguments cannot contain newlines.' }
        '"' + $argument.Replace('\', '/').Replace('"', '\"') + '"'
    }
    [IO.File]::WriteAllLines($Path, $quoted, $encoding)
}

$rustVersion = Invoke-Captured 'rust-version' $rustc @('-vV')
$llvmVersion = Invoke-Captured 'llvm-version' $llvmCov @('--version')
$rustLlvm = [regex]::Match($rustVersion, 'LLVM version:\s*([0-9.]+)').Groups[1].Value
$toolLlvm = [regex]::Match($llvmVersion, 'LLVM version\s+([0-9.]+)').Groups[1].Value
if (-not $rustLlvm -or $rustLlvm -ne $toolLlvm) { throw "LLVM tools must match rustc: rustc=$rustLlvm tools=$toolLlvm" }

. $PSScriptRoot/workspace.ps1
$SpiralCompiler = Copy-EoieCompilerSnapshot -Compiler $SpiralCompiler -Destination (Join-Path $work 'compiler')
$snapshot = & $PSScriptRoot/test-source-package.ps1 -EoieRoot $EoieRoot -EoieBinary $EoieBinary -SkipBuild -PassThru | ForEach-Object {
    if ($_.PSObject.Properties.Name -contains 'Manifest') { $_ } else { Write-Host $_ }
}
if (@($snapshot).Count -ne 1) { throw 'Source snapshot did not return one manifest.' }
$source = $snapshot.RestoredRoot
$profiles = Join-Path $work 'profiles'
[void][IO.Directory]::CreateDirectory($profiles)
$binary = Join-Path $TargetDirectory "release/eoie$suffix"
$settings = @{
    CARGO_BUILD_JOBS = [string]$Jobs; EOIE_COVERAGE_JOBS = [string]$Jobs; RUST_TEST_THREADS = '1'
    CARGO_INCREMENTAL = '1'; RUSTFLAGS = '-C instrument-coverage --cfg eoie_coverage'; CARGO_ENCODED_RUSTFLAGS = $null
    CARGO_TARGET_DIR = $TargetDirectory; CARGO_PROFILE_RELEASE_STRIP = 'none'
    LLVM_PROFILE_FILE = (Join-Path $profiles '%p-%m.profraw'); EOIE_SPIRAL_COMPILE = $SpiralCompiler
    EOIE_BIN_UNDER_TEST = $binary; EOIE_COVERAGE_SMOKE_CHILD = $null; RAYON_NUM_THREADS = [string]$Jobs
}
$previous = @{}
try {
    foreach ($name in $settings.Keys) {
        $previous[$name] = [Environment]::GetEnvironmentVariable($name)
        if ($null -eq $settings[$name]) { Remove-Item -LiteralPath "Env:$name" -ErrorAction SilentlyContinue }
        else { [Environment]::SetEnvironmentVariable($name, $settings[$name]) }
    }
    if (Test-Path Env:CARGO_ENCODED_RUSTFLAGS) { throw 'Encoded Cargo flags must be absent so instrumentation is effective.' }
    # Compile first with dependency build-script profiles kept outside runtime evidence.
    # Both subsequent Cargo invocations use this same frozen source and configuration.
    $buildProfiles = Join-Path $work 'build-profiles'
    [void][IO.Directory]::CreateDirectory($buildProfiles)
    $env:LLVM_PROFILE_FILE = Join-Path $buildProfiles '%p-%m.profraw'
    $manifest = Join-Path $source 'src/Cargo.toml'
    $buildArgs = @('build', '--manifest-path', $manifest, '--release', '--offline', '--locked', '--package', 'eoie-cli', '--bin', 'eoie', '--jobs', [string]$Jobs)
    [void](Invoke-Captured 'coverage-build' $cargo $buildArgs)
    $testArgs = @('test', '--manifest-path', $manifest, '--release', '--offline', '--locked', '--workspace', '--lib', '--bins', '--tests', '--jobs', [string]$Jobs)
    [void](Invoke-Captured 'coverage-test-build' $cargo ($testArgs + @('--no-run')))
    $env:LLVM_PROFILE_FILE = Join-Path $profiles '%p-%m.profraw'
    $run = Invoke-Captured 'workspace-run' $EoieBinary @('proxy', 'coverage-run', $source, $cargo, $SpiralCompiler, $TargetDirectory, $profiles, 'workspace', [string]$TimeoutMs)
    foreach ($line in ($run -split [char]10 | Where-Object { $_ -match 'test result: ok\. [1-9]|eoie coverage phase=|coverage-run ok' })) { Write-Host $line }
    if ($CompilerContracts) {
        $ignored = Invoke-Captured 'compiler-contracts' $cargo ($testArgs + @('--', '--ignored'))
        foreach ($line in ($ignored -split [char]10 | Where-Object { $_ -match 'test result: ok\. [1-9]' })) { Write-Host $line }
    }
    $artifacts = Invoke-Captured 'test-artifacts' $cargo ($testArgs + @('--no-run', '--message-format=json'))
    $objects = @($binary) + @($artifacts -split [char]10 | Where-Object { $_.StartsWith('{') } | ForEach-Object { $_ | ConvertFrom-Json } |
        Where-Object { $_.reason -eq 'compiler-artifact' -and $_.profile.test -and $_.executable } | ForEach-Object { $_.executable })
    $objects = @($objects | Sort-Object -Unique)
    if ($objects.Count -lt 2) { throw 'Cargo did not report test executables.' }
    $rawProfiles = @(Get-ChildItem -LiteralPath $profiles -File -Filter '*.profraw' | Sort-Object FullName)
    if (-not $rawProfiles.Count) { throw 'No native coverage profiles were produced.' }
    # LLVM may combine incompatible unused-function maps across test executables.
    # Match raw profiles to the runtime's module signature before exporting each object.
    $sourceManifest = Get-Content -LiteralPath $snapshot.Manifest -Raw | ConvertFrom-Json
    $sourceArguments = @('--sources') + @($sourceManifest | Where-Object { $_.Path -like 'src/*.rs' } | ForEach-Object { Join-Path $source $_.Path })
    $exports = [Collections.Generic.List[object]]::new()
    $lcovInputs = [Collections.Generic.List[string]]::new()
    $claimed = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    [void][IO.Directory]::CreateDirectory((Join-Path $work 'objects'))
    foreach ($object in $objects) {
        $id = '{0:d3}' -f $exports.Count
        $objectRoot = Join-Path $work "objects/$id"
        [void][IO.Directory]::CreateDirectory($objectRoot)
        $previousProfile = $env:LLVM_PROFILE_FILE
        try {
            $env:LLVM_PROFILE_FILE = Join-Path $objectRoot '%m.profraw'
            $probeArgs = if ($object -eq $binary) { @('help') } else { @('--list') }
            [void](Invoke-Captured "probe-$id" $object $probeArgs -RejectStderr -Quiet)
        } finally { $env:LLVM_PROFILE_FILE = $previousProfile }
        $probes = @(Get-ChildItem -LiteralPath $objectRoot -File -Filter '*.profraw')
        if ($probes.Count -ne 1) { throw "Expected one runtime module signature for $object; found $($probes.Count)." }
        $signature = $probes[0].Name
        $matching = @($rawProfiles | Where-Object { $_.Name.EndsWith('-' + $signature, [StringComparison]::Ordinal) })
        if (-not $matching.Count) { throw "No test-run profiles match the module signature of $object." }
        foreach ($profile in $matching) { [void]$claimed.Add($profile.FullName) }
        $inputList = Join-Path $objectRoot 'profiles.txt'
        [IO.File]::WriteAllLines($inputList, [string[]](@($matching.FullName) + @($probes[0].FullName)), $encoding)
        $merged = Join-Path $objectRoot 'coverage.profdata'
        [void](Invoke-Captured "merge-$id" $profdata @('merge', '--sparse', "--num-threads=$Jobs", "--input-files=$inputList", "--output=$merged") -RejectStderr -Quiet)
        $responseFile = Join-Path $objectRoot 'export.rsp'
        Write-LlvmArguments $responseFile (@('--format=lcov', "--num-threads=$Jobs", "--instr-profile=$merged", "--object=$object") + $sourceArguments)
        $lcov = Invoke-Captured "export-$id" $llvmCov @('export', "@$responseFile") -RejectStderr -Quiet
        if ($lcov -notmatch '(?m)^DA:') { throw "No workspace line coverage was exported for $object." }
        $relative = "objects/$id/current.lcov"
        [IO.File]::WriteAllText((Join-Path $work $relative), $lcov, $encoding)
        $lcovInputs.Add($relative)
        $exports.Add([pscustomobject]@{
            Object = $object; ObjectSHA256 = (Get-FileHash -LiteralPath $object -Algorithm SHA256).Hash
            ModuleSignature = $signature; Profiles = $matching.Count; Lcov = $relative
            LcovSHA256 = (Get-FileHash -LiteralPath (Join-Path $work $relative) -Algorithm SHA256).Hash
        })
        if ($exports.Count % 10 -eq 0 -or $exports.Count -eq $objects.Count) { Write-Host "Exported $($exports.Count)/$($objects.Count) native coverage objects." }
    }
    $unclaimed = @($rawProfiles | Where-Object { -not $claimed.Contains($_.FullName) } | ForEach-Object Name)
    if ($unclaimed.Count) { throw "Profiles outside the Cargo executable inventory require review: $($unclaimed -join ', ')" }
    $lcovPath = Join-Path $work 'current.lcov'
    [void](Invoke-Captured 'union-lcov' $EoieBinary (@('proxy', 'coverage-union', 'current.lcov') + $lcovInputs.ToArray()) -RejectStderr)
    $assessment = Invoke-Captured 'assess-lcov' $EoieBinary @('proxy', 'coverage-assess', $lcovPath, $lcovPath, '0', '0', '0')
    Write-Host $assessment.Trim()
    $evidence = [ordered]@{
        Schema = 1; Platform = [Runtime.InteropServices.RuntimeInformation]::OSDescription
        CompilerContracts = [bool]$CompilerContracts; Jobs = $Jobs; TestThreads = 1
        BuildProfiles = @(Get-ChildItem -LiteralPath $buildProfiles -File -Filter '*.profraw').Count
        BuildProfilePolicy = 'Compilation-only profiles are retained separately and excluded from runtime source coverage.'
        PreflightProfiles = @(Get-ChildItem -LiteralPath (Join-Path $profiles 'preflight') -File -Filter '*.profraw').Count
        PreflightPolicy = 'Early CLI instrumentation probes are separate because Cargo may relink the binary before tests.'
        SourceManifest = $snapshot.Manifest; SourceManifestSHA256 = (Get-FileHash -LiteralPath $snapshot.Manifest -Algorithm SHA256).Hash
        RustVersion = $rustVersion.Trim(); LlvmVersion = $llvmVersion.Trim()
        EoieSHA256 = (Get-FileHash -LiteralPath $EoieBinary -Algorithm SHA256).Hash
        DriverSHA256 = (Get-FileHash -LiteralPath $PSCommandPath -Algorithm SHA256).Hash
        CompilerSHA256 = (Get-FileHash -LiteralPath $SpiralCompiler -Algorithm SHA256).Hash
        ProfdataSHA256 = (Get-FileHash -LiteralPath $profdata -Algorithm SHA256).Hash
        LlvmCovSHA256 = (Get-FileHash -LiteralPath $llvmCov -Algorithm SHA256).Hash
        Profiles = $rawProfiles.Count; Objects = $exports.ToArray(); ProfileMatching = 'runtime module signature'; MergePolicy = 'maximum hits per source line'; Discovery = 'CLI help and test --list'; LcovSHA256 = (Get-FileHash -LiteralPath $lcovPath -Algorithm SHA256).Hash
        Assessment = $assessment.Trim(); CoverageValidated = $true; StrictReleaseCertification = $false
    }
    [IO.File]::WriteAllText((Join-Path $work 'evidence.json'), ($evidence | ConvertTo-Json -Depth 5), $encoding)
    Write-Host "EOIE native coverage passed: evidence=$(Join-Path $work 'evidence.json')"
} finally {
    foreach ($name in $previous.Keys) {
        if ($null -eq $previous[$name]) { Remove-Item -LiteralPath "Env:$name" -ErrorAction SilentlyContinue }
        else { [Environment]::SetEnvironmentVariable($name, $previous[$name]) }
    }
}
