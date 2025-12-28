/* ============================================================================
 * FILE: src/types.ts
 * MODULE: AI
 * ROLE:
 *   - Definitie van gedeelde types binnen de AI-module
 *
 * TRUST LEVEL:
 *   - NEUTRAL (compile-time only)
 *
 * RULES:
 *   - Bevat ENKEL types
 *   - GEEN runtime code
 *   - GEEN imports van logica
 * ============================================================================
 */

export type ChatRole = "system" | "user" | "assistant";

export type ChatMessage = {
  role: ChatRole;
  content: string;
};

export type AskRequest = {
  prompt: string;
};

export type ChatRequest = {
  messages: ChatMessage[];
};

export type AiResponse = {
  text: string;
  model: string;
};
