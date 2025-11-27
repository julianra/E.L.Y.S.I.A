🚀 E.L.Y.S.I.A. Roadmap (Fase 1 → Fase 4)

We splitsen het in 4 logische fases:

🟦 FASE 1 — Lokale basis stabiel krijgen

(waar je nu bent)

✔️ 1. Kernel stabiliseren

Database

Migraties

Modules auto-discovery

Route-registratie

Event bus

Logging

Boot sequence

👉 klaar

✔️ 2. Orbit (Flutter) basis

Projectstructuur klaar

Kan kernel status testen (/health)

Kan tasks sturen naar Marthe

👉 momenteel bezig

✔️ 3. Marthe basis-API

Implementeren:

POST /marthe/add_task

Database insert

Test data opvragen

✔️ 4. Orbit UI: taken toevoegen & lijst tonen

Textfield → taak toevoegen

Lijst van taken ophalen

TaskCard widget

JSON → models → UI

✔️ 5. Lokale discovery

Kernel broadcast via mDNS: elysia.local

Orbit → automatische detectie

Fallback: manual server input

Opslaan in SharedPreferences

👉 Dit is extreem belangrijk voor gebruiksgemak.

🟩 FASE 2 — Echte functionaliteit bouwen
✔️ 1. Datamodel Marthe afwerken

Een agenda-item bevat:

naam

beschrijving

starttijd

eindtijd

duur

categorie

project

energiebelasting

prioriteit

locatie

type (taak, afspraak, deadline)

recurring

AI properties (importanceScore, emotionalLoad)

We maken een schema:

marthe_tasks
marthe_timeblocks
marthe_recurrence
marthe_logs

✔️ 2. API uitbreiden voor Orbit

/marthe/tasks (get all)

/marthe/task/{id} (details)

/marthe/delete/{id}

/marthe/update/{id}

✔️ 3. Orbit UI

3 views:

Dag

Week

Maand

Net zoals Google Calendar + jouw eigen twist.

✔️ 4. Slimme planner — eerste versie

conflict detectie

slimme suggesties wanneer te plannen

notificaties

categorievolgorde

energieprofiel meenemen

deadlines automatisch verspreiden (time blocking)

✔️ 5. J.U.N.K Analytics basic integratie

meten van gebruik

trends herkennen

voorspellen hoe lang je taken DOEN er echt duren

betere planning

🟧 FASE 3 — Lokale multi-node + edge computing
✔️ 1. Kernel + modules op meerdere devices

Laptop

Raspberry Pi

Oude pc

Linux server

NAS

✔️ 2. Orbit → server select scherm

automatisch detecteren

manueel toevoegen

nodes een naam geven: “Thuis”, “Kantoor”, “Atelier”

✔️ 3. Node-communicatie

Eventbus uitbreiden over netwerk

Module-capabilities uitwisselen

Syncing tussen nodes

Dn krijg je bv:

ELYSIA-Home → CATNIP
ELYSIA-PC → MARTHE
ELYSIA-NAS → JUNK

Orbit kan kiezen welke server hij bestuurt.

🟪 FASE 4 — Remote Veiligheid + B2B + ELYSIA OS
✔️ 1. Remote access (super veilig)

Niet nu — maar hier.

Opties:

Tailscale (simpel & ultraveilig → 10 minuten setup)

ELYSIA Cloud Relay (professioneel)

Caddy reverse proxy + HTTPS

Token-based authentication

End-to-end encryptie

✔️ 2. ELYSIA Supervisor

Beheer van:

updates

modules installeren

backup / restore

log viewer

systeemstatus

gebruikersaccounts

✔️ 3. ELYSIA OS Distro

Gebaseerd op:

Alpine Linux

Arch + systemd

Buildroot (embedded)

Wordt een OS zoals Home Assistant OS:

Flashable image

OTA-updates

Node discovery

Modules geïsoleerd

Logging

I/O drivers

Dashboard

✔️ 4. ELYSIA Node Hardware (commercieel)

Je eigen product:

ELYSIA Home Node

ELYSIA Pro Node

ELYSIA Micro Node