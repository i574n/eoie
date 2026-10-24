param(
    $fast,
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"


$url = git ls-remote --get-url
$owner = ($url -split '/' | Select-Object -Last 2 | Select-Object -First 1) -replace '\.git$', '' ?? $env:GITHUB_REPOSITORY_OWNER
$domain = ($url -split '/' | Select-Object -Last 3 | Select-Object -First 1) ?? $env:GITHUB_SERVER_URL -replace 'https?://', ''
Write-Output "init.ps1 / url: $url / owner: $owner / domain: $domain"

Set-Location (New-Item -ItemType Directory -Path "../.." -Force)
if (!(Test-Path "spiral/.git")) {
    git clone --recurse-submodules https://$domain/$owner/spiral.git
    if ($LASTEXITCODE -ne 0) { throw "init.ps1 / git clone spiral failed (exit code $LASTEXITCODE)" }
}
Set-Location $ScriptDir

. ../../spiral/scripts/core.ps1

{ pwsh ../../spiral/scripts/init-app-build.ps1 -fast 1 } | Invoke-Block

{ mix local.hex --force } | Invoke-Block -Location ../../spiral/apps/cells
{ mix local.rebar --force } | Invoke-Block -Location ../../spiral/apps/cells
{ mix deps.get } | Invoke-Block -Location ../../spiral/apps/cells
