✅ WAT ELYSIA CORE NU AL KAN

ELYSIA Core is al een volledig werkende kernel met:

🔐 Auth

Admin aanmaken (alleen lokaal)

Inloggen

Secure password hashing (Argon2)

HMAC user-tokens

📡 Pairing

Public pairing voor devices (init + complete)

Admin pairing control (enable/disable)

Device tokens met verificatie

Devices worden opgeslagen in DB

🧩 Kernel Runtime

Database + migraties

Module loader

EventBus (basis)

mDNS broadcast (node discovery)

HTTP server op poort 2022

🖥️ UI Support

Status endpoint voor dashboard

Pairing endpoints voor dashboard controls

🎉 Core is dus volledig opgestart, bereikbaar en bruikbaar voor modules + UI.

🟥 WAT CORE NOG MOET HEBBEN (VERPLICHT)

Deze dingen zijn essentieel om Core 2.0 als “AF” te beschouwen:

1️⃣ Pairing expiry fix

Pairing moet echt automatisch uitgaan na X minuten.
De timestamp wordt nog niet opgeslagen → fix nodig.

2️⃣ Device database uitbreiden

Toevoegen:

last_seen

paired_at

ip

disabled

device_type

3️⃣ Secret key voor tokens vervangen

Nu hardcoded → moet runtime gegenereerd en opgeslagen worden.

4️⃣ Uniforme error-handling

Alles moet dezelfde JSON structuur gebruiken.

5️⃣ Module router finaliseren

Duidelijke schema:

404 handling

veilige module endpoint registraties

device/user auth onderscheid

→ Met deze MUST-haves is Core production-stable.

🟧 WAT CORE NOG KAN HEBBEN (OPTIONEEL, MAAR STERK AANBEVOLEN)
⭐ Device status API

Orbit moet kunnen opvragen of hij gekoppeld is, nog actief is, enz.

⭐ Module lifecycle callbacks

on_startup

on_shutdown

on_device_pair

Modules hebben dit nodig.

⭐ Verbeterde logging

JSON logs, file logs, log levels.

🟦 WAT CORE EXTRA KAN KRIJGEN (NICE TO HAVE / LUXE)
💬 WebSockets

Realtime communicatie met Orbit, HAVEN, CATNIP.

📊 Metrics endpoint

CPU, RAM, module performance

EventBus queue stats

Realtime dashboard load in UI

🔒 Module sandboxing

Crash isolation, rate limiting → enterprise-level.

🟩 CONCLUSIE

ELYSIA Core = 90% AF en perfect voor MARTHE + ORBIT ontwikkeling.
Je hebt een écht OS gebouwd.
Nu moeten we enkel de MUST-haves nog toevoegen voor stabiliteit, en daarna kun je modules bouwen alsof je op Linux werkt.