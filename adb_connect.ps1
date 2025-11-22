# ===========================================
# ADB Wireless Auto-Connect Script (PowerShell)
# ===========================================

Write-Host "📱 ADB Wireless Auto-Connector" -ForegroundColor Cyan

# Check if adb exists
if (-not (Get-Command adb -ErrorAction SilentlyContinue)) {
    Write-Host "❌ ADB not found. Install Android Platform Tools." -ForegroundColor Red
    exit
}

# 1. Check USB device
Write-Host "🔎 Checking USB devices..."
$devices = adb devices | Select-String "device$"

if ($devices.Count -eq 0) {
    Write-Host "❌ No USB Android device detected." -ForegroundColor Red
    Write-Host "➡ Connect your phone via USB first."
    exit
}

Write-Host "✔ USB device detected.`n"

# 2. Enable wireless mode
Write-Host "🔄 Enabling TCP/IP mode on port 5555..."
adb tcpip 5555
Start-Sleep -Seconds 2

# 3. Detect phone IP
Write-Host "🔎 Detecting phone IP..."
$ip = adb shell ip route | Select-String -Pattern "src" | ForEach-Object {
    ($_ -split "src ")[1]
}

if (-not $ip) {
    Write-Host "⚠ Unable to auto-detect IP." -ForegroundColor Yellow
    $ip = Read-Host "Enter phone IP manually"
}

Write-Host "📡 IP detected: $ip`n"

# 4. Try normal connect
Write-Host "🔗 Connecting wirelessly..."
$result = adb connect "$ip`:5555"

if ($result -match "connected") {
    Write-Host "🎉 SUCCESS! Phone connected wirelessly." -ForegroundColor Green
    exit
}

# 5. Wireless pairing fallback
Write-Host "⚠ Normal connect failed → using wireless pairing..." -ForegroundColor Yellow

$pairPort = Read-Host "Pairing port (shown on your phone)"
$pairCode = Read-Host "Pairing code"

Write-Host "🔐 Pairing..."
adb pair "$ip`:$pairPort" $pairCode

Write-Host "🔗 Connecting again..."
adb connect "$ip`:5555"

Write-Host "🎉 DONE!" -ForegroundColor Green
