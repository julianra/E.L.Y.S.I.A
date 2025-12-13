// src/schemas.ts

export type AITaskType =
  | "generate"
  | "summarize"
  | "classify"
  | "extract";

export interface AIRequest {
  task: AITaskType;
  input: string;
  context?: Record<string, string>;
}

export interface AIResponse {
  success: boolean;
  output: string;
  tokens: number;
}
