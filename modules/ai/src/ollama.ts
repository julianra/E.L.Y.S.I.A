/* ============================================================================
 * FILE: src/ollama.ts
 * MODULE: AI
 * ROLE:
 *   - Technische adapter naar de Ollama HTTP API
 *
 * TRUST LEVEL:
 *   - UNTRUSTED (extern LLM backend)
 *
 * RULES:
 *   - Kent GEEN HTTP server
 *   - Kent GEEN kernel
 *   - Kent GEEN permissies
 *   - Laadt GEEN environment config
 *
 * FAILURE MODE:
 *   - Gooit errors door naar de caller
 * ============================================================================
 */

import type { ChatMessage } from "./types.js";

function requireEnv(name: string): string {
  const value = process.env[name];
  if (!value) {
    throw new Error(`Missing environment variable: ${name}`);
  }
  return value;
}

export async function ollamaReachable(): Promise<boolean> {
  const baseUrl = requireEnv("OLLAMA_BASE_URL");

  try {
    const res = await fetch(`${baseUrl}/api/tags`);
    return res.ok;
  } catch {
    return false;
  }
}

export async function ollamaAsk(prompt: string): Promise<{ text: string; model: string }> {
  return ollamaChat([{ role: "user", content: prompt }]);
}

export async function ollamaChat(
  messages: ChatMessage[]
): Promise<{ text: string; model: string }> {
  const baseUrl = requireEnv("OLLAMA_BASE_URL");
  const model = requireEnv("OLLAMA_MODEL");
  const timeoutMs = Number(process.env.REQUEST_TIMEOUT_MS ?? "60000");

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const res = await fetch(`${baseUrl}/api/chat`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      signal: controller.signal,
      body: JSON.stringify({
        model,
        stream: false,
        messages
      })
    });

    if (!res.ok) {
      const text = await safeText(res);
      throw new Error(`Ollama HTTP ${res.status}: ${text}`);
    }

    const data = (await res.json()) as any;
    const text: string | undefined = data?.message?.content;

    if (!text) {
      throw new Error("Invalid or empty response from Ollama");
    }

    return { text, model };
  } finally {
    clearTimeout(timeout);
  }
}

async function safeText(res: Response): Promise<string> {
  try {
    return await res.text();
  } catch {
    return "<unreadable body>";
  }
}
