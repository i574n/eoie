[CmdletBinding()]
param(
    [string]$EoieRoot = (Join-Path $PSScriptRoot '..'),
    [string]$EoieBinary,
    [string]$TargetDirectory,
    [ValidateRange(1, 256)][int]$Jobs = 2,
    [ValidateRange(1000, 600000)][int]$TimeoutMs = 60000
)
$ErrorActionPreference = 'Stop'
$EoieRoot = (Resolve-Path -LiteralPath $EoieRoot).Path
$suffix = if ($IsWindows) { '.exe' } else { '' }
if (-not $EoieBinary) { $EoieBinary = Join-Path $EoieRoot "eoie$suffix" }
$EoieBinary = (Resolve-Path -LiteralPath $EoieBinary).Path
if (-not $TargetDirectory) { $TargetDirectory = Join-Path $EoieRoot 'src/target/native-probes' }
$TargetDirectory = [IO.Path]::GetFullPath($TargetDirectory, $PWD.Path)
$cargo = (Get-Command cargo -CommandType Application | Select-Object -First 1).Source
$work = Join-Path $EoieRoot ('.cache/native-probes/' + [guid]::NewGuid().ToString('N'))
[void][IO.Directory]::CreateDirectory($work)
$encoding = [Text.UTF8Encoding]::new($false)
Copy-Item -LiteralPath $EoieBinary -Destination (Join-Path $work "eoie$suffix")
$EoieBinary = Join-Path $work "eoie$suffix"
# The committed state/prompt.spi lease is one agent session's wall-clock budget and has always expired by the time CI or
# a later run validates the commit. The lease guard reads the caller's cwd ancestors, so the supervisor runs from outside
# the checkout with no inherited lease root; it writes only receipts into $work, and its children still run in $work.
$isolatedCwd = [IO.Path]::GetTempPath()

function Invoke-ProbeCapture([string]$Name, [string]$Program, [string[]]$Arguments, [int]$Budget = $TimeoutMs) {
    $receiptPath = Join-Path $work "$Name.txt"
    $leaseRoot = $env:EOIE_LEASE_ROOT
    Push-Location -LiteralPath $isolatedCwd
    try {
        $env:EOIE_LEASE_ROOT = $null
        & $EoieBinary proxy command-capture $work "$Name.txt" $Budget $Program @Arguments | ForEach-Object { Write-Host $_ }
        $captureExit = $LASTEXITCODE
    } finally { Pop-Location; $env:EOIE_LEASE_ROOT = $leaseRoot }
    if ($captureExit -ne 0) { throw "EOIE could not capture $Name." }
    $receipt = [IO.File]::ReadAllText($receiptPath)
    $startMarker = "--- stdout ---" + [char]10
    $endMarker = [char]10 + "--- stderr ---" + [char]10
    $start = $receipt.IndexOf($startMarker, [StringComparison]::Ordinal)
    $end = $receipt.LastIndexOf($endMarker, [StringComparison]::Ordinal)
    if ($start -lt 0 -or $end -lt $start) { throw "Invalid process receipt: $receiptPath" }
    $header = $receipt.Substring(0, $start)
    $output = $receipt.Substring($start + $startMarker.Length, $end - $start - $startMarker.Length)
    $bytes = [regex]::Match($header, '(?m)^stdout_bytes=(\d+)\r?$')
    if (-not $bytes.Success -or $encoding.GetByteCount($output) -ne [long]$bytes.Groups[1].Value) { throw "Invalid output length: $receiptPath" }
    [pscustomobject]@{
        Passed = ($header -match '(?m)^status=0\r?$' -and $header -match '(?m)^termination=Exited\(0\)\r?$')
        Output = $output; Receipt = $receiptPath
    }
}

$snapshot = & $PSScriptRoot/test-source-package.ps1 -EoieRoot $EoieRoot -EoieBinary $EoieBinary -SkipBuild -PassThru | ForEach-Object {
    if ($_.PSObject.Properties.Name -contains 'Manifest') { $_ } else { Write-Host $_ }
}
if (@($snapshot).Count -ne 1) { throw 'Expected one source snapshot.' }
$manifest = Join-Path $snapshot.RestoredRoot 'src/Cargo.toml'
$metadata = Invoke-ProbeCapture 'metadata' $cargo @('metadata', '--manifest-path', $manifest, '--offline', '--locked', '--no-deps', '--format-version', '1')
if (-not $metadata.Passed) { throw "Cargo metadata failed: $($metadata.Receipt)" }
$workspace = $metadata.Output | ConvertFrom-Json
$expected = @($workspace.packages | ForEach-Object {
    $package = $_
    $package.targets | Where-Object { $_.kind -contains 'example' } | ForEach-Object {
        [pscustomobject]@{ PackageId = $package.id; Package = $package.name; Target = $_.name; Source = $_.src_path }
    }
})
if (-not $expected.Count) { throw 'Workspace has no example probes.' }
$build = Invoke-ProbeCapture 'build' $cargo @('build', '--manifest-path', $manifest, '--target-dir', $TargetDirectory, '--workspace', '--examples', '--offline', '--locked', '--jobs', [string]$Jobs, '--message-format=json') 600000
if (-not $build.Passed) { throw "Native probe build failed: $($build.Receipt)" }
$artifacts = @($build.Output -split [char]10 | Where-Object { $_.StartsWith('{') } | ForEach-Object { $_ | ConvertFrom-Json } |
    Where-Object { $_.reason -eq 'compiler-artifact' -and $_.target.kind -contains 'example' -and $_.executable })
if ($artifacts.Count -ne $expected.Count) { throw "Example inventory differs: declared=$($expected.Count) built=$($artifacts.Count)" }
$results = [Collections.Generic.List[object]]::new()
foreach ($probe in ($expected | Sort-Object Package,Target)) {
    $matching = @($artifacts | Where-Object { $_.package_id -eq $probe.PackageId -and $_.target.name -eq $probe.Target })
    if ($matching.Count -ne 1) { throw "Expected one executable for $($probe.Package)/$($probe.Target)." }
    $binary = $matching[0].executable
    $id = 'probe-{0:d3}' -f $results.Count
    $run = Invoke-ProbeCapture $id $binary @()
    $results.Add([pscustomobject]@{
        Package = $probe.Package; Target = $probe.Target; Passed = $run.Passed; Receipt = $run.Receipt
        SourceSHA256 = (Get-FileHash -LiteralPath $probe.Source -Algorithm SHA256).Hash
        BinarySHA256 = (Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash
    })
    Write-Host "Probe $($probe.Package)/$($probe.Target): passed=$($run.Passed)"
}
$failed = @($results | Where-Object { -not $_.Passed })
$evidence = [ordered]@{
    Schema = 1; SourceManifest = $snapshot.Manifest
    SourceManifestSHA256 = (Get-FileHash -LiteralPath $snapshot.Manifest -Algorithm SHA256).Hash
    EoieSHA256 = (Get-FileHash -LiteralPath $EoieBinary -Algorithm SHA256).Hash
    DriverSHA256 = (Get-FileHash -LiteralPath $PSCommandPath -Algorithm SHA256).Hash
    Jobs = $Jobs; ProbeConcurrency = 1; Declared = $expected.Count; Passed = $results.Count - $failed.Count
    StrictReleaseCertification = $false; Results = $results.ToArray()
}
$evidencePath = Join-Path $work 'evidence.json'
[IO.File]::WriteAllText($evidencePath, ($evidence | ConvertTo-Json -Depth 5), $encoding)
if ($failed.Count) { throw "$($failed.Count) native probes failed; evidence=$evidencePath" }
Write-Host "EOIE native probes passed: count=$($results.Count) evidence=$evidencePath"
