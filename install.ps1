$InstallDir = "$env:LOCALAPPDATA\interpreter"
$AliasDir   = "$env:LOCALAPPDATA\Microsoft\WindowsApps"

#place holder for actual name
Write-Host "Installing interpreter..."

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item -Force "$PSScriptRoot\interpreter.exe" $InstallDir

$aliasPath = "$AliasDir\interpreter.cmd"
"@`"$InstallDir\interpreter.exe`" %*" | Out-File -Encoding ASCII $aliasPath

#adding path variable
$path = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($path -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$path;$InstallDir", "User")
}

Write-Host "interpreter installed! You can now run 'interpreter' from any terminal."