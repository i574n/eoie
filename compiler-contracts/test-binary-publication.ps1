[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'workspace.ps1')
$fixture = Join-Path ([IO.Path]::GetTempPath()) ('eoie publication ü ' + [guid]::NewGuid().ToString('N'))
[void][IO.Directory]::CreateDirectory($fixture)
$source = Join-Path $fixture ('candidate' + $(if ($IsWindows) { '.exe' } else { '' }))
$destination = Join-Path $fixture ('installed' + $(if ($IsWindows) { '.exe' } else { '' }))
$previousMode = $env:EOIE_PUBLICATION_TEST_MODE
function Assert-Publication([bool]$Condition, [string]$Message) { if (-not $Condition) { throw $Message } }
function Assert-Rejected([string]$Label, [string]$Mode, [string]$InputPath = $source, [int]$Budget = 1000, [string]$Root) {
    $env:EOIE_PUBLICATION_TEST_MODE = $Mode
    [IO.File]::WriteAllText($destination, 'previous executable')
    $before = (Get-FileHash -LiteralPath $destination).Hash
    $rejected = $false
    try { [void](Publish-EoieBinary -Source $InputPath -Destination $destination -TimeoutMs $Budget -ValidationRoot $Root) } catch { $rejected = $true }
    Assert-Publication $rejected "$Label was accepted."
    Assert-Publication ((Get-FileHash -LiteralPath $destination).Hash -ceq $before) "$Label replaced the previous executable."
    Assert-Publication (@(Get-ChildItem -LiteralPath $fixture -Filter '.release-eoie-candidate*' -Force).Count -eq 0) "$Label leaked a candidate."
    Write-Host "PASS $Label preserves the previous executable"
}
try {
    $rust = Join-Path $fixture 'candidate.rs'
    [IO.File]::WriteAllText($rust, @'
use std::{env, thread, time::Duration};
fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.first().map(String::as_str) == Some("proxy") {
        assert_eq!(args.len(), 3);
        assert_eq!(args[1], "source-topology");
        match env::var("EOIE_PUBLICATION_TEST_MODE").unwrap_or_default().as_str() {
            "topology-exit" => std::process::exit(2),
            "topology-output" => println!("topology_limits_ok=false"),
            "topology-timeout" => thread::sleep(Duration::from_secs(30)),
            _ => println!("topology_limits_ok=true owner_growth_policy=report-only"),
        }
        return;
    }
    assert_eq!(args, ["help", "--schema"]);
    match env::var("EOIE_PUBLICATION_TEST_MODE").unwrap_or_default().as_str() {
        "exit" => std::process::exit(3),
        "schema" => println!("not an EOIE schema"),
        "timeout" => thread::sleep(Duration::from_secs(30)),
        _ => println!("command-spec"),
    }
}
'@, [Text.UTF8Encoding]::new($false))
    & rustc --crate-name publication_candidate $rust -o $source
    if ($LASTEXITCODE -ne 0) { throw 'Cannot compile publication fixture.' }
    $env:EOIE_PUBLICATION_TEST_MODE = 'good'
    Assert-Publication (Publish-EoieBinary -Source $source -Destination $destination) 'First install was not published.'
    Assert-Publication ((Get-FileHash -LiteralPath $source).Hash -ceq (Get-FileHash -LiteralPath $destination).Hash) 'Installed bytes differ.'
    Write-Host 'PASS first install validates and publishes the candidate'
    $stamp = [IO.File]::GetLastWriteTimeUtc($destination)
    Assert-Publication (-not (Publish-EoieBinary -Source $source -Destination $destination)) 'Identical binary was republished.'
    Assert-Publication ([IO.File]::GetLastWriteTimeUtc($destination) -eq $stamp) 'Identical binary timestamp changed.'
    Write-Host 'PASS identical binaries preserve timestamps'
    Assert-Rejected 'nonzero candidate exit' 'exit'
    Assert-Rejected 'invalid candidate schema' 'schema'
    Assert-Rejected 'candidate timeout' 'timeout' -Budget 100
    $invalid = Join-Path $fixture 'invalid.exe'
    [IO.File]::WriteAllText($invalid, 'not an executable')
    Assert-Rejected 'invalid executable' 'good' -InputPath $invalid
    Assert-Rejected 'topology rejection' 'topology-exit' -Root $fixture
    Assert-Rejected 'invalid topology output' 'topology-output' -Root $fixture
    Assert-Rejected 'topology timeout' 'topology-timeout' -Root $fixture -Budget 100
    $env:EOIE_PUBLICATION_TEST_MODE = 'good'
    Assert-Publication (Publish-EoieBinary -Source $source -Destination $destination -ValidationRoot $fixture) 'Validated topology was not published.'
    Write-Host 'PASS publication requires the requested topology gate'
    if ($IsWindows) {
        $env:EOIE_PUBLICATION_TEST_MODE = 'good'
        [IO.File]::WriteAllText($destination, 'locked executable')
        $handle = [IO.File]::Open($destination, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
        $rejected = $false
        try {
            try { [void](Publish-EoieBinary -Source $source -Destination $destination) } catch { $rejected = $true }
            Assert-Publication $rejected 'Locked destination was replaced.'
            Assert-Publication ([IO.File]::ReadAllText($destination) -ceq 'locked executable') 'Locked destination changed.'
        } finally { $handle.Dispose() }
        Write-Host 'PASS a Windows destination lock preserves the installed bytes'
    }
    $env:EOIE_PUBLICATION_TEST_MODE = 'good'
    [IO.File]::WriteAllText($destination, 'stale executable')
    Assert-Publication (Publish-EoieBinary -Source $source -Destination $destination) 'Replacement was not published.'
    Assert-Publication ((Get-FileHash -LiteralPath $source).Hash -ceq (Get-FileHash -LiteralPath $destination).Hash) 'Replacement bytes differ.'
    Assert-Publication (@(Get-ChildItem -LiteralPath $fixture -Filter '.release-eoie-candidate*' -Force).Count -eq 0) 'Publication leaked candidates.'
    Write-Host 'PASS validated replacement cleans its temporary candidate'
    Write-Host 'EOIE binary publication contracts passed.'
} finally {
    if ($null -eq $previousMode) { Remove-Item Env:EOIE_PUBLICATION_TEST_MODE -ErrorAction SilentlyContinue }
    else { $env:EOIE_PUBLICATION_TEST_MODE = $previousMode }
    $resolved = [IO.Path]::GetFullPath($fixture)
    $temporaryRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath()).TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
    if (-not $resolved.StartsWith($temporaryRoot, [StringComparison]::OrdinalIgnoreCase)) { throw "Unsafe fixture cleanup: $resolved" }
    if (Test-Path -LiteralPath $resolved) { Remove-Item -LiteralPath $resolved -Recurse -Force }
}
