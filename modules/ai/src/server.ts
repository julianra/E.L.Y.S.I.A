/* ============================================================================
 * FILE: src/server.ts
 * MODULE: AI
 * ROLE:
 *   - HTTP interface van de AI-module
 *   - Valideert inkomende requests
 *   - Routeert requests naar de Ollama adapter
 *
 * TRUST LEVEL:
 *   - UNTRUSTED (extern moduleproces)
 *
 * RULES:
 *   - Accepteert momenteel requests van iedereen (fase 0)
 *   - Bevat GEEN kernel-authenticatie
 *   - Bevat GEEN state of opslag
 *   - Bevat GEEN AI-logica
 *
 * STARTUP POLICY (BLOCKING):
 *   - Start pas met luisteren als embedded Ollama + phi3.5 ready is
 * ============================================================================
 */

import "dotenv/config";

import express from "express";
import { ollamaAsk, ollamaChat, ollamaReachable } from "./ollama.js";
import { ensureEmbeddedOllamaReady, registerOllamaShutdownHooks } from "./ollamaRuntime.js";
import type { AskRequest, ChatRequest, AiResponse } from "./types.js";

const app = express();
app.use(express.json({ limit: "1mb" }));

const port = Number(process.env.PORT ?? "8787");

/* -------------------------------------------------------------------------- */
/* Health check                                                               */
/* -------------------------------------------------------------------------- */

app.get("/health", async (_req, res) => {
  const ok = await ollamaReachable();

  res.status(ok ? 200 : 503).json({
    ok,
    ollamaBaseUrl: process.env.OLLAMA_BASE_URL ?? null,
    model: process.env.OLLAMA_MODEL ?? null
  });
});

/* -------------------------------------------------------------------------- */
/* Simple ask endpoint                                                        */
/* -------------------------------------------------------------------------- */

app.post("/v1/ask", async (req, res) => {
  try {
    const body = req.body as AskRequest;

    if (!body || typeof body.prompt !== "string" || body.prompt.trim() === "") {
      return res.status(400).json({
        error: "Invalid request body: expected { prompt: string }"
      });
    }

    const result = await ollamaAsk(body.prompt.trim());

    const response: AiResponse = {
      text: result.text,
      model: result.model
    };

    return res.json(response);
  } catch (err: any) {
    return res.status(500).json({
      error: err?.message ?? "Unknown error"
    });
  }
});

/* -------------------------------------------------------------------------- */
/* Chat endpoint                                                              */
/* -------------------------------------------------------------------------- */

app.post("/v1/chat", async (req, res) => {
  try {
    const body = req.body as ChatRequest;

    if (!body || !Array.isArray(body.messages) || body.messages.length === 0) {
      return res.status(400).json({
        error: "Invalid request body: expected { messages: [...] }"
      });
    }

    for (const msg of body.messages) {
      if (
        typeof msg !== "object" ||
        typeof (msg as any).role !== "string" ||
        typeof (msg as any).content !== "string"
      ) {
        return res.status(400).json({
          error: "Invalid message format"
        });
      }
    }

    const result = await ollamaChat(body.messages);

    const response: AiResponse = {
      text: result.text,
      model: result.model
    };

    return res.json(response);
  } catch (err: any) {
    return res.status(500).json({
      error: err?.message ?? "Unknown error"
    });
  }
});

/* -------------------------------------------------------------------------- */
/* Startup                                                                    */
/* -------------------------------------------------------------------------- */

async function main(): Promise<void> {
  // Embedded Ollama (simpel): forceer fixed host+model in env voor deze module.
  // Dit houdt de rest van de code model-agnostisch.
  process.env.OLLAMA_BASE_URL = process.env.OLLAMA_BASE_URL ?? "http://127.0.0.1:11434";
  process.env.OLLAMA_MODEL = "phi3.5";

  registerOllamaShutdownHooks();

  // Blokkerend klaarzetten: ollama serve + pull phi3.5 indien nodig
  await ensureEmbeddedOllamaReady();

  app.listen(port, () => {
    console.log(`[AI] module listening on http://127.0.0.1:${port}`);
  });
}

main().catch((e) => {
  console.error(`[AI] startup failed: ${e?.message ?? e}`);
  process.exit(1);
});
