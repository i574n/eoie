param(
    $ScriptDir = $PSScriptRoot
)
Set-Location $ScriptDir
$ErrorActionPreference = "Stop"
. ../../spiral/scripts/core.ps1


{ pwsh ../../spiral/scripts/publish-tree.ps1 -Root .. -Exclude '.cache' } | Invoke-Block
