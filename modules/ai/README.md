# ELYSIA – AI Module (Standalone)

Standalone AI-module voor ELYSIA.
Deze module fungeert als proxy tussen ELYSIA en Ollama.

## Eigenschappen
- Standalone HTTP service
- Geen kernelkennis
- Geen permissies
- Geen pairing
- Geen state

## Endpoints
- GET /health
- POST /v1/ask
- POST /v1/chat

## Doel
Eerst standalone werken.
Daarna stap voor stap hardenen zodat enkel de Kernel toegang heeft.
