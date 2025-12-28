/* ============================================================================
 * FILE: src/ollamaRuntime.ts
 * MODULE: AI
 * ROLE:
 *   - Beheer van embedded Ollama runtime (start/health/model bootstrap)
 *   - Garandeert dat Ollama HTTP API bereikbaar is vóór de server luistert
 *
 * TRUST LEVEL:
 *   - UNTRUSTED (extern proces)
 *
 * RULES:
 *   - GEEN HTTP endpoints (dat is server.ts)
 *   - GEEN kernel-auth (komt later)
 *   - Alleen lifecycle + readiness checks
 *
 * DESIGN (SIMPEL):
 *   - Windows-first
 *   - Hardcoded host: 127.0.0.1:11434
 *   - Hardcoded model: phi3.5
 *   - Blokkerend wachten: server start pas als ready
 * ============================================================================
 */

import { spawn, type ChildProcess } from "node:child_process";
import { mkdirSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const OLLAMA_HOST = "127.0.0.1:11434";
const OLLAMA_BASE_URL = `http://${OLLAMA_HOST}`;
const REQUIRED_MODEL = "phi3.5";

// We houden een reference bij zodat we bij shutdown netjes kunnen killen.
let ollamaProc: ChildProcess | null = null;

export type EnsureOllamaOptions = {
  startupTimeoutMs?: number; // totale tijd om serve + model ready te krijgen
  pollIntervalMs?: number;   // hoe vaak we /api/tags pingen
};

export async function ensureEmbeddedOllamaReady(opts: EnsureOllamaOptions = {}): Promise<void> {
  if (process.platform !== "win32") {
    // Simpel houden: alleen Windows nu.
    throw new Error("Embedded Ollama is currently implemented for Windows only (win32).");
  }

  const startupTimeoutMs = Number(opts.startupTimeoutMs ?? 180_000); // 3 min
  const pollIntervalMs = Number(opts.pollIntervalMs ?? 500);

  // 1) Als er al een Ollama luistert: gebruik die (maar je systeem is ollama-vrij, dus normaal niet).
  const reachable = await isOllamaReachable();
  if (!reachable) {
    // 2) Start embedded ollama serve
    startEmbeddedOllamaServe();
  }

  // 3) Wacht tot HTTP API leeft
  await waitUntilReachable(startupTimeoutMs, pollIntervalMs);

  // 4) Zorg dat phi3.5 aanwezig is
  const hasModel = await modelExists(REQUIRED_MODEL);
  if (!hasModel) {
    await pullModel(REQUIRED_MODEL, startupTimeoutMs);
  }
}

export function registerOllamaShutdownHooks(): void {
  // Zorgt dat we onze embedded ollama opruimen als de module stopt.
  const shutdown = () => {
    try {
      if (ollamaProc && !ollamaProc.killed) {
        ollamaProc.kill();
      }
    } catch {
      // bewust stil
    } finally {
      process.exit(0);
    }
  };

  process.on("SIGINT", shutdown);
  process.on("SIGTERM", shutdown);
}

function startEmbeddedOllamaServe(): void {
  const exePath = getEmbeddedOllamaExePath();
  const { modelsDir } = getEmbeddedOllamaDataDirs();

  mkdirSync(modelsDir, { recursive: true });

  // Belangrijk: forceer binding op localhost + vaste poort
  const env = {
    ...process.env,
    OLLAMA_HOST,
    OLLAMA_MODELS: modelsDir
  };

  // Ollama "serve" als achtergrondproces
  const child = spawn(exePath, ["serve"], {
    env,
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true
  });

  child.stdout.on("data", () => {
    // bewust niet loggen: simpel houden
  });
  child.stderr.on("data", () => {
    // bewust niet loggen: simpel houden
  });

  child.on("exit", (code) => {
    // Als Ollama onverwacht stopt, is de module onbruikbaar.
    // We laten server.ts dit via /health zichtbaar maken, maar omdat we blocking startup doen,
    // zou dit normaal pas gebeuren na succesvolle start.
    ollamaProc = null;
    void code;
  });

  ollamaProc = child;
}

function getEmbeddedOllamaExePath(): string {
  // We resolven relatief t.o.v. dist/ zodat dit ook werkt in distributie.
  // dist/ollamaRuntime.js -> projectRoot (modules/ai)
  const here = path.dirname(fileURLToPath(import.meta.url)); // dist
  const projectRoot = path.resolve(here, ".."); // modules/ai
  return path.join(projectRoot, "runtime", "ollama", "win", "ollama.exe");
}

function getEmbeddedOllamaDataDirs(): { modelsDir: string } {
  const here = path.dirname(fileURLToPath(import.meta.url)); // dist
  const projectRoot = path.resolve(here, ".."); // modules/ai
  const modelsDir = path.join(projectRoot, "runtime", "ollama", "models");
  return { modelsDir };
}

async function isOllamaReachable(): Promise<boolean> {
  try {
    const res = await fetch(`${OLLAMA_BASE_URL}/api/tags`, { method: "GET" });
    return res.ok;
  } catch {
    return false;
  }
}

async function waitUntilReachable(timeoutMs: number, pollIntervalMs: number): Promise<void> {
  const start = Date.now();

  while (true) {
    const ok = await isOllamaReachable();
    if (ok) return;

    if (Date.now() - start > timeoutMs) {
      throw new Error(`Ollama did not become reachable within ${timeoutMs}ms on ${OLLAMA_BASE_URL}`);
    }

    await sleep(pollIntervalMs);
  }
}

async function modelExists(modelPrefix: string): Promise<boolean> {
  const res = await fetch(`${OLLAMA_BASE_URL}/api/tags`, { method: "GET" });
  if (!res.ok) return false;

  const data = (await res.json()) as any;
  const models: Array<{ name?: string }> = Array.isArray(data?.models) ? data.models : [];
  return models.some((m) => typeof m?.name === "string" && m.name.toLowerCase().startsWith(modelPrefix.toLowerCase()));
}

async function pullModel(model: string, timeoutMs: number): Promise<void> {
  // Gebruik de HTTP API om te pullen (simpel, geen extra child process).
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const res = await fetch(`${OLLAMA_BASE_URL}/api/pull`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      signal: controller.signal,
      body: JSON.stringify({
        name: model,
        stream: false
      })
    });

    if (!res.ok) {
      const txt = await safeText(res);
      throw new Error(`Failed to pull model '${model}': HTTP ${res.status}: ${txt}`);
    }

    // Sommige versies geven JSON terug met status; we hoeven dat niet te verwerken voor MVP.
    await res.arrayBuffer();
  } finally {
    clearTimeout(timer);
  }

  // Verifieer dat model er nu is.
  const ok = await modelExists(model);
  if (!ok) {
    throw new Error(`Model '${model}' was pulled but not present in /api/tags afterwards.`);
  }
}

async function safeText(res: Response): Promise<string> {
  try {
    return await res.text();
  } catch {
    return "<unreadable body>";
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}
