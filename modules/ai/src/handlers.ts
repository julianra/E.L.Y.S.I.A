// src/handlers.ts

import { AIRequest, AIResponse } from "./schemas";
import { callAI } from "./client";

export async function handleAIRequest(
  req: AIRequest
): Promise<AIResponse> {
  // Geen policy hier
  // Geen module checks hier
  // Alleen uitvoeren

  return await callAI(req);
}
