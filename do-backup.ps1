$baseDir = [Environment]::GetFolderPath('MyPictures')
$backupDir = Join-Path $baseDir 'AuraBackup'
Write-Host "AuraBackup path: $backupDir"

try {
    Push-Location $backupDir
    # aura-exporter backup sync --save-dir . --delay-ms 3000 --jiggle-ms 3000 --refresh-every 5m --debug
    aura-exporter backup sync --save-dir . --delay-ms 3000 --jiggle-ms 3000 --refresh-every 1h
} finally {
    Pop-Location
}