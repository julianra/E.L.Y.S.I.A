// =========================================================
// 📍 FILE: modules/ai/src/schemas.ts
// 📝 ROLE:
//   Contracts Kernel <-> AI-module
// =========================================================

export type AITask = "generate";

export interface AIExecuteRequest {
  task: AITask;
  input: string;
  context?: Record<string, string>;
}

export interface AIExecuteResponse {
  success: boolean;
  output: string;
  tokens: number;
  error?: string | null;
}
