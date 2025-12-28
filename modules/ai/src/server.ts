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
 * FUTURE:
 *   - Kernel-only access via headers
 *   - Signed requests + nonce
 *   - Rate limiting per kernel identity
 * ============================================================================
 */

/* -------------------------------------------------------------------------- */
/* Environment setup                                                          */
/* -------------------------------------------------------------------------- */

import "dotenv/config";

/* -------------------------------------------------------------------------- */
/* Imports                                                                    */
/* -------------------------------------------------------------------------- */

import express from "express";
import { ollamaAsk, ollamaChat, ollamaReachable } from "./ollama.js";
import type { AskRequest, ChatRequest, AiResponse } from "./types.js";

/* -------------------------------------------------------------------------- */
/* App setup                                                                  */
/* -------------------------------------------------------------------------- */

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
        typeof msg.role !== "string" ||
        typeof msg.content !== "string"
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

app.listen(port, () => {
  console.log(`[AI] module listening on http://127.0.0.1:${port}`);
});
