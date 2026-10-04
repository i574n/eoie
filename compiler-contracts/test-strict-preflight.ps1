[CmdletBinding()]
param(
    [string]$EoieRoot = (Join-Path $PSScriptRoot '..'),
    [string]$EoieBinary,
    [switch]$RequireReady,
    [switch]$PassThru
)
$ErrorActionPreference = 'Stop'
$EoieRoot = (Resolve-Path -LiteralPath $EoieRoot).Path
$suffix = if ($IsWindows) { '.exe' } else { '' }
if (-not $EoieBinary) { $EoieBinary = Join-Path $EoieRoot "eoie$suffix" }
$EoieBinary = (Resolve-Path -LiteralPath $EoieBinary).Path
$work = Join-Path $EoieRoot ('.cache/strict-preflight/' + [guid]::NewGuid().ToString('N'))
$release = Join-Path $work 'release'
[void][IO.Directory]::CreateDirectory($release)
Copy-Item -LiteralPath $EoieBinary -Destination (Join-Path $work "supervisor$suffix")
$EoieBinary = Join-Path $work "supervisor$suffix"

$snapshot = & $PSScriptRoot/test-source-package.ps1 -EoieRoot $EoieRoot -EoieBinary $EoieBinary -SkipBuild -PassThru | ForEach-Object {
    if ($_.PSObject.Properties.Name -contains 'Manifest') { $_ } else { Write-Host $_ }
}
if (-not $snapshot -or $snapshot.Count -gt 1) { throw 'Expected one verified source snapshot.' }
$manifest = Get-Content -Raw -LiteralPath $snapshot.Manifest | ConvertFrom-Json
$excluded = @($manifest | Where-Object { $_.Path -eq 'src/.gitignore' } | ForEach-Object { $_.Path })
$releaseFiles = @($manifest | Where-Object { $_.Path -match '^(src|state|evidence)/' -and $_.Path -notin $excluded })
$count = 0
foreach ($file in $releaseFiles) {
    $destination = Join-Path $release $file.Path
    [void][IO.Directory]::CreateDirectory((Split-Path $destination -Parent))
    Copy-Item -LiteralPath (Join-Path $snapshot.RestoredRoot $file.Path) -Destination $destination
    if ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash -cne $file.SHA256) { throw "Staging hash mismatch: $($file.Path)" }
    $count++
}
[void][IO.Directory]::CreateDirectory((Join-Path $release 'evidence/coverage'))
Copy-Item -LiteralPath $EoieBinary -Destination (Join-Path $release "eoie$suffix")

& $EoieBinary proxy command-capture $work 'strict-check.txt' 120000 $EoieBinary bundle check $release eoie | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Strict preflight supervision failed.' }
$receiptPath = Join-Path $work 'strict-check.txt'
$receipt = [IO.File]::ReadAllText($receiptPath)
$headerEnd = $receipt.IndexOf("--- stdout ---" + [char]10, [StringComparison]::Ordinal)
if ($headerEnd -lt 0) { throw 'Strict preflight process receipt is malformed.' }
$header = $receipt.Substring(0, $headerEnd)
$ready = $header -match '(?m)^status=0\r?$' -and $header -match '(?m)^termination=Exited\(0\)\r?$'
$diagnostics = @($receipt -split '\r?\n' | Where-Object { $_ -match '^(bundle error:|eoie error:)' })
# Recheck every staged source: inspection must not rewrite receipts or source files.
foreach ($file in $releaseFiles) {
    if ((Get-FileHash -LiteralPath (Join-Path $release $file.Path) -Algorithm SHA256).Hash -cne $file.SHA256) {
        throw "Strict preflight mutated staged source: $($file.Path)"
    }
}
$result = [pscustomobject]@{
    Schema = 1
    SourceManifest = $snapshot.Manifest
    SourceManifestSHA256 = (Get-FileHash -LiteralPath $snapshot.Manifest -Algorithm SHA256).Hash
    EoieSHA256 = (Get-FileHash -LiteralPath $EoieBinary -Algorithm SHA256).Hash
    DriverSHA256 = (Get-FileHash -LiteralPath $PSCommandPath -Algorithm SHA256).Hash
    ReleaseRoot = $release
    StagedSourceFiles = $count
    RepositoryOnlyFiles = $excluded
    CheckPassed = [bool]$ready
    StrictReleaseCertification = $false
    EvidenceRenewed = $false
    Receipt = $receiptPath
    Diagnostics = $diagnostics
}
$report = Join-Path $work 'report.json'
[IO.File]::WriteAllText($report, ($result | ConvertTo-Json -Depth 5), [Text.UTF8Encoding]::new($false))
Write-Host "EOIE strict preflight: ready=$ready diagnostics=$($diagnostics.Count) report=$report"
foreach ($diagnostic in $diagnostics) { Write-Host $diagnostic }
if ($RequireReady -and -not $ready) { throw "Strict release preflight rejected this snapshot; see $report" }
if ($PassThru) { $result }
