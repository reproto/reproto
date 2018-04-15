# Build and package for windows
param([string]$version = $(throw "-version is required"))

if (-not (Get-Command Compress-7Zip -errorAction SilentlyContinue)) {
    Write-Host "installing: 7Zip4PowerShell"
    Install-Package -Scope CurrentUser -Force 7Zip4PowerShell
}

$platform = "windows"
$arch = "unknown"

switch ($ENV:PROCESSOR_ARCHITECTURE) {
    "AMD64" { $arch = "x86_64" }
    default {
        throw "unrecognized arch: PROCESSOR_ARCHITECTURE = $ENV:PROCESSOR_ARCHITECTURE"
    }
}

$tuple = "$version-$platform-$arch"

Write-Host "arch: $arch, platform: $platform"

$exe = ".\target\release\reproto.exe"

if (-not (Test-Path $exe)) {
    throw "no such path: $exe"
}

Compress-7Zip -ArchiveFileName reproto-$tuple.tar -Path $exe
Compress-7Zip -ArchiveFileName reproto-$tuple.tar.gz -Path reproto-$tuple.tar

Remove-Item -path reproto-$tuple.tar

Write-Host "built: reproto-$tuple.tar.gz"