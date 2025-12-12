# =====================================================================
#  build-elysia.ps1
#  Kernel build + UI (Tauri) build + Starter build
#  Uit te voeren vanuit de map /elysia
# =====================================================================

$ErrorActionPreference = "Stop"

# -------------------------------------------------------------
# CONFIG
# -------------------------------------------------------------
$KernelProjectPath  = "kernel"
$KernelBinaryName   = "kernel.exe"

$StarterProjectPath = "starter"
$StarterBinaryName  = "elysia_starter.exe"

$CoreOutputPath     = "core"
$CoreOutputFile     = "core.exe"

$UiOutputPath       = "ui"
$UiOutputFile       = "ui.exe"

$uiRoot = "kernel_ui"

Write-Host "`n=== ELYSIA BUILD SCRIPT START ===" -ForegroundColor Cyan

# -------------------------------------------------------------
# 1. Cleanup UI
# -------------------------------------------------------------
Write-Host "`n[1/6] Cleanup UI..." -ForegroundColor Yellow
function Invoke-CargoCleanWithRetry {
    param(
        [string]$Path,
        [int]$MaxRetries = 10
    )

    for ($i = 1; $i -le $MaxRetries; $i++) {
        try {
            Write-Host " - cargo clean in $Path (attempt $i/$MaxRetries)"
            Push-Location $Path
            cargo clean
            Pop-Location
            Write-Host "   -> OK"
            return
        }
        catch {
            Write-Host "   Failed: $($_.Exception.Message)"
            if ($i -eq $MaxRetries) {
                throw "cargo clean failed after $MaxRetries attempts"
            }
            
            Write-Host "   Waiting 1s before retry..."
            Start-Sleep -Seconds 1
        }
    }
}

$pathsToRemove = @(
    "$uiRoot\.svelte-kit",
    "$uiRoot\build",
    "$uiRoot\output",
    "$uiRoot\src-tauri\gen",
    "$uiRoot\src-tauri\resources\core.exe"   # core in resources
)

foreach ($p in $pathsToRemove) {
    if (Test-Path $p) {
        Remove-Item $p -Recurse -Force -ErrorAction SilentlyContinue
        Write-Host " - Removed $p"
    }
}

# -------------------------------------------------------------
# 2. Cargo clean (kernel + UI)
# -------------------------------------------------------------
Write-Host "`n[2/6] Cargo clean..." -ForegroundColor Yellow

if (Test-Path "$KernelProjectPath\Cargo.toml") {
    Invoke-CargoCleanWithRetry -Path $KernelProjectPath
}

if (Test-Path "$uiRoot\src-tauri\Cargo.toml") {
    Invoke-CargoCleanWithRetry -Path "$uiRoot\src-tauri"
}


# -------------------------------------------------------------
# 3. Kernel build
# -------------------------------------------------------------
Write-Host "`n[3/6] Kernel builden..." -ForegroundColor Yellow

Push-Location $KernelProjectPath
cargo build --release
Pop-Location

$kernelExe = "target\release\$KernelBinaryName"

if (-not (Test-Path $kernelExe)) {
    throw "Kernel binary niet gevonden: $kernelExe"
}

Write-Host " - Kernel build OK → $kernelExe"

# Outputmap /core
if (-not (Test-Path $CoreOutputPath)) {
    New-Item -ItemType Directory -Path $CoreOutputPath | Out-Null
}

Copy-Item $kernelExe "$CoreOutputPath\$CoreOutputFile" -Force
Write-Host " - Copied kernel → $CoreOutputPath\$CoreOutputFile"

# Kopie naar Tauri resources
$tauriRes = "$uiRoot\src-tauri\resources"
if (-not (Test-Path $tauriRes)) { 
    New-Item -ItemType Directory $tauriRes | Out-Null 
}

Copy-Item $kernelExe "$tauriRes\core.exe" -Force
Write-Host " - Copied kernel → resources/core.exe (Tauri bundle)"

# -------------------------------------------------------------
# 4. UI build
# -------------------------------------------------------------
Write-Host "`n[4/6] UI builden..." -ForegroundColor Yellow

Push-Location $uiRoot

if (-not (Test-Path "node_modules")) {
    Write-Host " - node_modules ontbreekt → npm install..."
    npm install
}

npm run build
npm run tauri build

Pop-Location

$uiExe = "$uiRoot\src-tauri\target\release\kernel_ui.exe"

if (-not (Test-Path $uiExe)) {
    throw "UI executable niet gevonden: $uiExe"
}

if (-not (Test-Path $UiOutputPath)) {
    New-Item -ItemType Directory -Path $UiOutputPath | Out-Null
}

Copy-Item $uiExe "$UiOutputPath\$UiOutputFile" -Force
Write-Host " - Copied UI → $UiOutputPath\$UiOutputFile"

# -------------------------------------------------------------
# 5. Starter build
# -------------------------------------------------------------
Write-Host "`n[5/6] Starter builden..." -ForegroundColor Yellow

if (Test-Path "$StarterProjectPath\Cargo.toml") {
    Push-Location $StarterProjectPath
    cargo build --release
    Pop-Location

    $starterExe = "$StarterProjectPath\target\release\$StarterBinaryName"

    if (Test-Path $starterExe) {
        Copy-Item $starterExe "elysia.exe" -Force
        Write-Host " - Starter OK → elysia.exe"
    } else {
        Write-Host " - Starter binary niet gevonden, overslaan." -ForegroundColor DarkYellow
    }
} else {
    Write-Host " - Geen starter-project aanwezig, overslaan." -ForegroundColor DarkYellow
}

# -------------------------------------------------------------
# 6. Done
# -------------------------------------------------------------
Write-Host "`n[6/6] DONE - De volgende bestanden zijn opgebouwd:" -ForegroundColor Green
Write-Host " * ./core/core.exe"
Write-Host " * ./ui/ui.exe"
Write-Host " * ./elysia.exe (indien starter aanwezig)"
Write-Host ""
Write-Host "=== ELYSIA BUILD COMPLETED ===" -ForegroundColor Cyan
Write-Host ""
