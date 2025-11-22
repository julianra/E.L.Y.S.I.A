🧠 MARTHE v2 — COMPLETE TODO LIST
✅ FASE 1 — Basisplanner (AFGEROND)

✔ binnenkomende taak parsen
✔ dag detecteren
✔ dag automatisch aanmaken
✔ taak koppelen aan dag
✔ bestaande taken ophalen
✔ eenvoudige tijdslijn berekenen
✔ start_time / end_time automatisch invullen
✔ taak opslaan
✔ debug logging

➡️ Dit werkt nu perfect.

🔥 FASE 2 — Conflictbehandeling (Volgende stap)
🟥 2.1 Conflict detectie

We moeten logica bouwen die:

checkt of start_time < bestaande end_time

checkt of end_time > bestaande start_time

overlap herkent

blokkerende regels respecteert

Todo:

 functie detect_conflicts_for_day(&day_id)

 return Vec<Conflict> (struct aanmaken)

 basic conflict type: tijd overlapt

 later extra conflicts: locatie, context, energie, tools

🔥 FASE 3 — Automatische replanning

Van zodra er conflicten zijn, gebeurt dit:

🟧 3.1 Taken verschuiven

 functie shift_tasks_after(day_id, time)
verplaatst alle taken die beginnen na een bepaald tijdstip.

🟧 3.2 Dag opnieuw opbouwen

De hele dag opnieuw berekenen:

 functie recompute_day_schedule(day_id)

 bestaande taken ophalen

 sorteren op prioriteit → start → deadline

 taken herplannen met onze baseline scheduler

🔥 FASE 4 — AI-interpretatie van de taak (zeer belangrijk)

We gebruiken AI om te begrijpen wat de taak eigenlijk is, zodat MARTHE betere beslissingen kan nemen.

🟦 4.1 AI-classificatie van taak

AI beantwoordt:

Waar moet deze taak plaatsvinden?

thuis

onderweg

buiten

kantoor

overal

Wanneer moet dit soort taak gebeuren?

ochtend / avond

voor vertrek

na thuiskomst

uitstellen tot weekend

ASAP

Wat is het energieniveau?

Wat is het type? (hygiëne, werk, focus, klusjes, administratie, bellen…)

Hoe zwaar voelt de taak aan? (emotional_load)

Todo:

 nieuwe endpoint in Kernel om AI te vragen: ask_ai_task_analysis(name)

 struct AiTaskAnalysis { ideal_time, location_type, category, energy_cost, emotional_load }

 toevoegen in AgendaPoint

Voorbeeld:

tanden poetsen
→ category: hygiene
→ location: home
→ ideal_time: waking / before leaving house
→ energy_cost: 1
→ emotional_load: 0


Voorbeeld:

jari bellen
→ category: social / communication
→ location: quiet environment
→ ideal_time: anytime except commute

🔥 FASE 5 — AI-gestuurde planning

Hier wordt MARTHE een echte automate-planner.

🟩 5.1 Prioriteiten op basis van AI

belangrijke taken eerst

lichte taken ergens tussen

mentale load spreiden

🟩 5.2 Energie-gebaseerde planning

zware taken niet om 22:00

creatieve taken niet om 06:00

moeilijke taken vroeg op de dag

AI geeft:

importance_score
confidence_score
predicted_duration
emotional_load
energy_cost

🔥 FASE 6 — Locatie-aware planning

taken die thuis moeten gebeuren → pas plannen als “home block”

werk-taken → in “work block”

klusjes buiten → clusteren op zaterdag

auto-samenhang → “als ik onderweg ben”

Todo:

 detecteren home/work/outside blocks

 context scheduler

 travel time integreren

 taken niet plannen als je op de bus zit

🔥 FASE 7 — Geavanceerde tijdsplanning

 vrije gaten (gaps) detecteren

 taak in beste slot zetten

 rekening houden met deadlines

 rekening houden met recurrent tasks

 rekening houden met buffertijden tussen taken

 “focus blocks" (geen taken hier plannen)

 automatische uitbreiding naar weken/maanden

🔥 FASE 8 — Flutter / Web integratie (Agenda UI)

 dagweergave

 weekweergave

 maandweergave

 taak-drag-and-drop

 kleuren per categorie

 energie-weergave

 filter (home / work / outside)

 AI-suggestions onderaan scherm

 “Replan day” button

🔥 FASE 9 — J.U.N.K analyse (data intelligence)

 analyse van taken per week

 detectie van te weinig slaap / te veel werk

 detectie van taken die te vaak uitgesteld worden

 voorspelling van toekomstige drukte

 energiebalans per dag

 AI-aanbevelingen voor planning

🔥 FASE 10 — ELYSIA autonomie

 ELYSIA voorspelt wanneer je moet vertrekken

 ELYSIA plant taken automatisch tussen afspraken

 ELYSIA geeft melding: “je energie is laag, verschuif X naar morgen?”

 ELYSIA vraagt context (“ben je thuis?”) en plant live verder

 ELYSIA doet replanning wanneer taken overruns hebben

 ELYSIA voorspelt je ritme op lange termijn

🧩 BONUS — Plugins zoals WhatsApp, SMS, Mail

AI kan detecteren: "jari bellen" → vraagt aan Catnip: mag ik bellen?

AI plant: “Beste moment om te bellen is rond 20:30, thuis.”

Voor mails → beter in ochtendblokken

Voor creatieve dingen → middag of voormiddag