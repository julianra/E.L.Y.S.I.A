# 🚀 E.L.Y.S.I.A. – MASTER TODO & PROJECTPLAN  
Versie v2025.12 – Inclusief CARE + SHIELD + Rust AI Kernel  
Status: Orbit Views ✔️ Afgewerkt

---

# 🟦 FASE 1 — Lokale Basis & Fundamenten  
(HUIDIGE FASE — bijna volledig klaar)

Doelen  
✔ Stabiliteit  
✔ Lokale kernel  
✔ Modules draaien  
✔ AI-kern werkend  
✔ Orbit verbinden  
✔ MARTHE basis  
✔ Agenda-weergaven ✔  
❗ Laatste kleine restpunten kernel/AI

---

## 1. Kernel (Rust) — **VOLTOOID (98%)**
Status: ✔ klaar

Wat is al gebeurd:
- ✔ Rust workspace  
- ✔ elysia_core  
- ✔ SQLite + migraties  
- ✔ Module discovery  
- ✔ Axum API  
- ✔ mDNS advertising  
- ✔ EventBus (sync+async)  
- ✔ Logging per module  
- ✔ Crash-vrije boot sequence  

Wat moet nog:
- ⬜ Kernel errors uitbreiden  
- ⬜ Global context store  
- ⬜ Event throttling & priority  
- ⬜ Unit tests  
- ⬜ Graceful shutdown  

---

## 2. Orbit (Flutter) — **Views compleet! (90%)**

Wat is al gebeurd:
- ✔ mDNS discovery  
- ✔ Verbindingstest  
- ✔ Server opslaan  
- ✔ AddTask UI  
- ✔ TaskList UI  
- ✔ MARTHE API werkt  
- ✔ Dagweergave  
- ✔ Weekweergave  
- ✔ Maandweergave  
- ✔ Jaarweergave  
- ✔ Scroll/zoom (gekeurd)  
- ✔ Dagdetail per uur  

Wat moet nog:
- ⬜ SHIELD alert pop-ups (Fase 2)  
- ⬜ CARE live transcript UI (Fase 2)  
- ⬜ Module dashboards  
- ⬜ CATNIP devicelijst  
- ⬜ Settingspages per module  

---

## 3. MARTHE (Rust module) — 40% klaar

Wat is al gebeurd:
- ✔ Module draait  
- ✔ POST /marthe/tasks  
- ✔ Taken opslaan  
- ✔ Basis AI parsing  
- ✔ SlotEngine skelet  

Wat moet nog:
- ⬜ SlotEngine volledig  
- ⬜ Tijd parsing standaardiseren  
- ⬜ Prioriteit/categorie/tags  
- ⬜ Conflict-detectie  
- ⬜ Automatische herschikking  
- ⬜ Routines (dag/week/maand)  
- ⬜ MARTHE personality (sarcastisch)  

---

## 4. AI-Kernel (Rust + Ollama) — 60% klaar

Wat is al gebeurd:
- ✔ Backend gekoppeld  
- ✔ AIIntent enum  
- ✔ AI router  
- ✔ AI tests  
- ✔ Promptformats vertaald  
- ✔ Lokaal model phi3.5/llama3  
- ✔ MARTHE-ParseTask werkt  

Wat moet nog:
- ⬜ Foutcodes uitbreiden  
- ⬜ Retry-logic  
- ⬜ Streaming support  
- ⬜ Kernel-memory  
- ⬜ DOODO context-injectie  
- ⬜ AiBackend trait uniform  

---

## 5. JUNK — 10% klaar (Rust migratie later in fase 2)

Wat is al gebeurd:
- ✔ Python versie bestond  
- ✔ Database structuur  
- ✔ AI reasoning in verleden  
- ✔ TF-IDF tools  
- ✔ Import scripts  

Wat moet nog:
- ⬜ Rust implementatie  
- ⬜ Memory collector  
- ⬜ Semantic indexer  
- ⬜ Audit trail  
- ⬜ TF-IDF Rust migratie  
- ⬜ Query API  

---

## 6. SCOUT — 0%  
(Fase 2/3)

## 7. PACK / ARC / ARIA — later (Fase 3/4)

---

# 🟩 FASE 2 — De Echte Functionaliteit  
(**Volgende stap nu Orbit views klaar zijn!**)

## 🔥 FASE 2 PRIORITEITSVOLGORDE (belangrijk!)
1️⃣ **CATNIP v1 — Sensor ingest / Devices**  
2️⃣ **SHIELD Basis — Alerts, thresholds, logging**  
3️⃣ **CARE Basis — Whisper, transcriptie, simplify**  
4️⃣ **JUNK Lite — logs & summaries**  
5️⃣ **MARTHE v2 — real planner**  

---

## Fase 2.1 — CATNIP v1 (Start NU)

Taken:
- ⬜ ESP32 discovery  
- ⬜ Sensor ingest (rook, gas, CO₂, water, temp/hum)  
- ⬜ Relay control  
- ⬜ OTA via PACK (simple)  
- ⬜ MQTT client of HTTP ingest  
- ⬜ Device status view in Orbit  
- ⬜ Kernel events dispatch  

---

## Fase 2.2 — SHIELD Basis (na CATNIP)

Taken:
- ⬜ Thresholds configureren via Orbit  
- ⬜ Alerts → Kernel  
- ⬜ Alerts → Orbit  
- ⬜ Logging naar JUNK  
- ⬜ Simple alarm mode (popup + tekst)  

---

## Fase 2.3 — CARE Basis

Taken:
- ⬜ Whisper integratie (Rust C-bindings of Python FFI)  
- ⬜ Live transcriptie → Orbit  
- ⬜ Simplify mode  
- ⬜ Dysfasie hulp  
- ⬜ Entity detection in transcript  
- ⬜ Logging naar JUNK  

---

## Fase 2.4 — JUNK Lite
- ⬜ TF-IDF migratie  
- ⬜ Daily summary  
- ⬜ Simpele logs import  
- ⬜ Query endpoint  

---

## Fase 2.5 — Orb UITBREIDING
- ⬜ SHIELD Alerts UI  
- ⬜ CARE Transcript Feed  
- ⬜ Module dashboards  
- ⬜ CATNIP Devices UI  
- ⬜ ROUTINES (MARTHE)  
- ⬜ Threshold UI (SHIELD)  

---

# 🟧 FASE 3 — Multi-Node, Automatisatie, Redundantie

(Volledige inhoud blijft identiek zoals in vorige versie)

---

# 🟪 FASE 4 — AI OS, AR, Assistive & Safety Revolution

(Volledige inhoud blijft identiek zoals in vorige versie)

---

# 🟧 EXTRA OVERZICHT — WAT IS AL 100% KLAAR?

Module | Status | Opmerking
-------|--------|-----------
Rust Kernel | ✔ 100% | basis compleet  
mDNS | ✔ 100% | werkt perfect  
Orbit basics | ✔ 90% | views volledig klaar  
Orbit views | ✔ 100% | dag/week/maand/jaar OK  
MARTHE skeleton | ✔ 60% | basis werkt  
AI-intent kernel | ✔ 70% | JSON output mooi  
AI-test | ✔ 100% | goed  
Ollama backend | ✔ 90% | puur refactor later  
SHIELD module | ❌ 0% | fase 2  
CARE module | ❌ 0% | fase 2  
CATNIP module | ❌ 0% | fase 2 startpunt  
SCOUT | ❌ 0% | fase 2/3  
JUNK rust | ❌ 0% | fase 2 later  
PACK/ARC/ARIA | ❌ | fase 3/4

