// =========================================================
// 📍 FILE: modules/ai/src/client.ts
// 📝 ROLE:
//   Low-level HTTP client naar elysia_ai
// =========================================================

import { AIExecuteRequest, AIExecuteResponse } from "./schemas";

const AI_BASE_URL = "http://127.0.0.1:8123";

export async function callAI(
  req: AIExecuteRequest
): Promise<AIExecuteResponse> {
  if (req.task !== "generate") {
    throw new Error(`Unsupported AI task: ${req.task}`);
  }

  const res = await fetch(`${AI_BASE_URL}/generate`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      input: req.input,
      context: req.context ?? {}
    })
  });

  if (!res.ok) {
    throw new Error(`AI service HTTP ${res.status}`);
  }

  return (await res.json()) as AIExecuteResponse;
}
