// =========================================================
// 📍 FILE: modules/ai/src/index.ts
// 📝 ROLE:
//   Exports voor Kernel
// =========================================================

import { callAI } from "./client";
import { AIExecuteRequest, AIExecuteResponse } from "./schemas";

async function execute(
  payload: AIExecuteRequest
): Promise<AIExecuteResponse> {
  return await callAI(payload);
}

export default {
  ai: {
    execute
  }
};
