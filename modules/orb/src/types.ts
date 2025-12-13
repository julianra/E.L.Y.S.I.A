export interface OrbContext {
  surface: string;
  page: string;
}

export interface OrbAskRequest {
  question: string;
  context?: OrbContext;
  mode: "single_turn";
}

export interface OrbAskResponse {
  ok: boolean;
  reply?: string;
  meta?: Record<string, unknown>;
  error?: string;
}
