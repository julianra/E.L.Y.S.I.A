// src/client.ts

import { AIRequest, AIResponse } from "./schemas";

const AI_ENDPOINT = "http://127.0.0.1:8123";

export async function callAI(req: AIRequest): Promise<AIResponse> {
  const response = await fetch(`${AI_ENDPOINT}/generate`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({
      input: req.input,
      context: req.context ?? {}
    })
  });

  if (!response.ok) {
    throw new Error(`AI service error: ${response.status}`);
  }

  return (await response.json()) as AIResponse;
}
