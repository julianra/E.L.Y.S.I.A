// =========================================================
// 📍 FILE: modules/orb/src/index.ts
// 🧠 MODULE: ORB
// 🎯 ROLE: Single-turn orchestration layer (Phase 1)
//
// Logging philosophy:
// - Be extremely explicit
// - Never assume downstream behavior
// - If something is silent, it is broken
// =========================================================

import express, { Request, Response } from "express";

const app = express();
app.use(express.json({ limit: "256kb" }));

const PORT = Number(process.env.PORT ?? 8130);
const CORE_URL = process.env.CORE_URL ?? "http://127.0.0.1:2022";

// ----------------------------------------------------------------------
// HEALTH
// ----------------------------------------------------------------------

app.get("/health", (_req: Request, res: Response) => {
  console.log("[ORB] /health check");
  res.json({ ok: true });
});

// ----------------------------------------------------------------------
// ORB ASK
// ----------------------------------------------------------------------

app.post("/orb/ask", async (req: Request, res: Response) => {
  console.log("[ORB] Incoming /orb/ask request");
  console.log("[ORB] Raw request body:", req.body);

  const { question, mode } = req.body ?? {};

  if (typeof question !== "string" || mode !== "single_turn") {
    console.warn("[ORB] Invalid request payload");
    return res.status(400).json({
      ok: false,
      error: "Invalid ORB request payload",
    });
  }

  const payload = {
    question,
    mode,
  };

  const coreUrl = `${CORE_URL}/ai/execute`;

  console.log("[ORB] Preparing Core request");
  console.log("[ORB] Core URL:", coreUrl);
  console.log("[ORB] Payload to Core:", payload);

  let coreResponse;
  let coreText;

  try {
    coreResponse = await fetch(coreUrl, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(payload),
    });

    console.log("[ORB] Core HTTP status:", coreResponse.status);

    coreText = await coreResponse.text();
    console.log("[ORB] Raw Core response body:", coreText);
  } catch (err) {
    console.error("[ORB] Failed to reach Core:", err);
    return res.status(502).json({
      ok: false,
      error: "Failed to reach Core",
    });
  }

  let coreJson: any = null;

  try {
    coreJson = JSON.parse(coreText);
    console.log("[ORB] Parsed Core JSON:", coreJson);
  } catch (err) {
    console.error("[ORB] Failed to parse Core JSON:", err);
    return res.status(502).json({
      ok: false,
      error: "Invalid JSON from Core",
    });
  }

  const reply =
    typeof coreJson.reply === "string" ? coreJson.reply : null;

  console.log("[ORB] Normalised reply:", reply);

  if (!reply) {
    console.warn("[ORB] No reply received from Core");
    return res.json({
      ok: false,
      reply: null,
    });
  }

  console.log("[ORB] Returning reply to caller");

  return res.json({
    ok: true,
    reply,
  });
});

// ----------------------------------------------------------------------
// START SERVER
// ----------------------------------------------------------------------

app.listen(PORT, () => {
  console.log(`[ORB] Listening on http://127.0.0.1:${PORT}`);
  console.log(`[ORB] CORE_URL = ${CORE_URL}`);
});
