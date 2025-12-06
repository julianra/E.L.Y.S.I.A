<#
=====================================================================================
🎯 ELYSIA MODULE GENERATOR SCRIPT — create_module.ps1 (V9 FINAL)
-------------------------------------------------------------------------------------
Genereert een volledige module in /modules/<module> inclusief:

  ✔ Premium NL headers
  ✔ PascalCase structnamen
  ✔ Record + CreateInput + Response modellen
  ✔ API, service, events, model, migrations
  ✔ Uiterst stabiel Replace → Set-Content systeem
=====================================================================================
#>

param(
    [Parameter(Mandatory=$true)]
    [string]$ModuleName
)

# =====================================================================================
# 📌 PAD-INFORMATIE & VARIABELEN
# =====================================================================================

$scriptRoot  = Split-Path -Parent $MyInvocation.MyCommand.Path
$modulesRoot = Join-Path $scriptRoot "modules"

$module      = $ModuleName.ToLower()
$className   = $module.Substring(0,1).ToUpper() + $module.Substring(1)
$modulePath  = Join-Path $modulesRoot $module
$timestamp   = (Get-Date).ToString("yyyy-MM-dd HH:mm:ss")

Write-Host "🚀 Nieuwe ELYSIA module wordt aangemaakt: $module" -ForegroundColor Cyan

if (Test-Path $modulePath) {
    Write-Host "❌ Module '$module' bestaat al!" -ForegroundColor Red
    exit 1
}

# =====================================================================================
# 📁 1. MAPPEN
# =====================================================================================

New-Item -ItemType Directory -Path $modulePath | Out-Null
New-Item -ItemType Directory -Path "$modulePath/src" | Out-Null
New-Item -ItemType Directory -Path "$modulePath/migrations" | Out-Null

# =====================================================================================
# 🔧 HEADER HELPERS
# =====================================================================================

function HeaderRS {
param($filePath, $className, $timestamp, $desc, $purpose)

$h = @'
/* ================================================================================
   E.L.Y.S.I.A. MODULE BESTAND — AUTOMATISCH GEGENEREERD
====================================================================================

   📄 BESTAND: FILEPATH
   🧩 MODULE: CLASSNAME
   🕒 GEGENEREERD OP: TIMESTAMP

------------------------------------------------------------------------------------
   📌 BESCHRIJVING:
       DESC

   🎯 DOEL:
       PURPOSE
------------------------------------------------------------------------------------

   ⚠️ NIET HANDMATIG AANPASSEN
   Dit bestand is automatisch gegenereerd door het ELYSIA module-generatiescript.
   Manuele wijzigingen aan deze header kunnen leiden tot inconsistentie.

==================================================================================== */
'@

$h = $h.Replace("FILEPATH", $filePath)
$h = $h.Replace("CLASSNAME", $className)
$h = $h.Replace("TIMESTAMP", $timestamp)
$h = $h.Replace("DESC", $desc)
$h = $h.Replace("PURPOSE", $purpose)
return $h
}

function HeaderSQL {
param($filePath, $className, $timestamp)

$h = @'
-- ================================================================================
--   E.L.Y.S.I.A. MODULE MIGRATIE — AUTOMATISCH GEGENEREERD
-- ================================================================================
--
--   📄 BESTAND: FILEPATH
--   🧩 MODULE: CLASSNAME
--   🕒 GEGENEREERD OP: TIMESTAMP
--
--   📌 BESCHRIJVING:
--       Dit migratiebestand initialiseert de databankstructuur voor de module.
--
--   🎯 DOEL:
--       Aanmaken van de basisdatabase-tabel die deze module gebruikt.
--
-- ================================================================================
'@

$h = $h.Replace("FILEPATH", $filePath)
$h = $h.Replace("CLASSNAME", $className)
$h = $h.Replace("TIMESTAMP", $timestamp)
return $h
}

# =====================================================================================
# 📄 2. Cargo.toml
# =====================================================================================

$toml = @"
# =============================================================================
# 📦 FILE: modules/$module/Cargo.toml
# =============================================================================

[package]
name = "$module"
version = "0.1.0"
edition = "2024"

[dependencies]
elysia_core = { path = "../../elysia_core" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
async-trait = "0.1"
axum = "0.7"
"@
Set-Content "$modulePath/Cargo.toml" $toml

# =====================================================================================
# 📄 3. lib.rs
# =====================================================================================

$header = HeaderRS "modules/$module/src/lib.rs" $className $timestamp `
"Initialisatiebestand van de ELYSIA-module." `
"Registratie, setup en verbinding van de module met ELYSIA Core."

$content = @'
HEADER

use async_trait::async_trait;
use elysia_core::{Module, ModuleContext, ModuleDescriptor, Result};
use inventory::submit;

pub mod api;
pub mod service;
pub mod model;
pub mod events;

pub struct CLASSNAME;

#[submit]
fn register() -> ModuleDescriptor {
    ModuleDescriptor::new("MODULE", || Box::new(CLASSNAME))
}

#[async_trait]
impl Module for CLASSNAME {
    async fn init(&self, ctx: &ModuleContext) -> Result<()> {
        ctx.register_api(api::routes(ctx.clone()));
        ctx.subscribe(events::handlers());
        Ok(())
    }
}
'@

$content = $content.Replace("HEADER", $header)
$content = $content.Replace("CLASSNAME", $className)
$content = $content.Replace("MODULE", $module)
Set-Content "$modulePath/src/lib.rs" $content

# =====================================================================================
# 📄 4. api.rs
# =====================================================================================

$header = HeaderRS "modules/$module/src/api.rs" $className $timestamp `
"Definitie van alle HTTP-routes van deze module." `
"Afhandelen van API-verzoeken en koppeling naar de service-logica."

$content = @'
HEADER

use axum::{Router, routing::post, Json};
use elysia_core::ModuleContext;
use serde::Deserialize;

use crate::service;

#[derive(Deserialize)]
pub struct ExampleInput {
    pub value: String,
}

pub fn routes(ctx: ModuleContext) -> Router {
    Router::new()
        .route("/m/MODULE/example", post(move |body| example(ctx.clone(), body)))
}

async fn example(ctx: ModuleContext, Json(payload): Json<ExampleInput>) -> Json<String> {
    let result = service::process_example(ctx, payload.value).await;
    Json(result)
}
'@

$content = $content.Replace("HEADER", $header)
$content = $content.Replace("MODULE", $module)
Set-Content "$modulePath/src/api.rs" $content

# =====================================================================================
# 📄 5. service.rs
# =====================================================================================

$header = HeaderRS "modules/$module/src/service.rs" $className $timestamp `
"Businesslogica van deze ELYSIA-module." `
"Verwerken van input, uitvoeren van functies en uitsturen van events."

$content = @'
HEADER

use elysia_core::ModuleContext;

pub async fn process_example(ctx: ModuleContext, value: String) -> String {
    ctx.emit("MODULE_example_event", value.clone()).await;
    format!("Processed in MODULE: {}", value)
}
'@

$content = $content.Replace("HEADER", $header)
$content = $content.Replace("MODULE", $module)
Set-Content "$modulePath/src/service.rs" $content

# =====================================================================================
# 📄 6. model.rs (Optie 3)
# =====================================================================================

$header = HeaderRS "modules/$module/src/model.rs" $className $timestamp `
"Datamodellen voor opslag, input en output van deze module." `
"Definieert structuren voor databaseopslag, API-input en API-output."

$content = @'
HEADER

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CLASSNAMERecord {
    pub id: i64,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CLASSNAMECreateInput {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CLASSNAMEResponse {
    pub id: i64,
    pub text: String,
}
'@

$content = $content.Replace("HEADER", $header)
$content = $content.Replace("CLASSNAME", $className)
Set-Content "$modulePath/src/model.rs" $content

# =====================================================================================
# 📄 7. events.rs
# =====================================================================================

$header = HeaderRS "modules/$module/src/events.rs" $className $timestamp `
"Event handlers van de module." `
"Verwerkt binnenkomende events vanuit ELYSIA Core of andere modules."

$content = @'
HEADER

use elysia_core::{Event, EventHandler};

pub fn handlers() -> Vec<EventHandler> {
    vec![
        EventHandler::new("NodeOnline", on_node_online),
    ]
}

async fn on_node_online(_event: Event) {
    println!("[MODULE] Node online detected");
}
'@

$content = $content.Replace("HEADER", $header)
$content = $content.Replace("MODULE", $module)
Set-Content "$modulePath/src/events.rs" $content

# =====================================================================================
# 📄 8. migration
# =====================================================================================

$header = HeaderSQL "modules/$module/migrations/001_init.sql" $className $timestamp

$content = @'
HEADER

CREATE TABLE IF NOT EXISTS MODULE_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    text TEXT NOT NULL
);
'@

$content = $content.Replace("HEADER", $header)
$content = $content.Replace("MODULE", $module)
Set-Content "$modulePath/migrations/001_init.sql" $content

# =====================================================================================
# 🎉 9. DONE
# =====================================================================================

Write-Output "================================================="
Write-Output "Module '$module' succesvol aangemaakt!"
Write-Output "Locatie: $modulePath"
Write-Output ""
Write-Output "Run nu: cargo build"
Write-Output "================================================="
