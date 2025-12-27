param(
    [Parameter(Mandatory = $true)][string]$Version,
    [Parameter(Mandatory = $true)][string]$Target,
    [Parameter(Mandatory = $true)][string]$TargetDir
)

$binName = "coco-local.exe"
$binPath = Join-Path $TargetDir $binName
$distDir = Join-Path $TargetDir "dist"
$archive = "coco-local-$Version-$Target.zip"

New-Item -ItemType Directory -Force -Path $distDir | Out-Null
Compress-Archive -Path $binPath -DestinationPath (Join-Path $distDir $archive) -Force

if ($env:GITHUB_OUTPUT) {
    "PACKAGE=$(Join-Path $distDir $archive)" | Out-File -FilePath $env:GITHUB_OUTPUT -Append
}
