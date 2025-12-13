# ======================================================================
# 📍 FILE: elysia_ai/main.py
# 🧠 SERVICE: ELYSIA AI
# 🎯 ROLE: Strict AI execution service with fixed response contract
# ======================================================================

from fastapi import FastAPI
from pydantic import BaseModel
import requests
import uvicorn

app = FastAPI()

OLLAMA_URL = "http://127.0.0.1:11434/api/generate"
MODEL = "phi3.5:latest"


# ----------------------------------------------------------------------
# REQUEST MODEL
# ----------------------------------------------------------------------

class GenerateRequest(BaseModel):
    input: str


# ----------------------------------------------------------------------
# RESPONSE MODEL (STRICT CONTRACT)
# ----------------------------------------------------------------------

class GenerateResponse(BaseModel):
    success: bool
    reply: str | None
    error: str | None
    meta: dict | None


# ----------------------------------------------------------------------
# GENERATE ENDPOINT
# ----------------------------------------------------------------------

@app.post("/generate", response_model=GenerateResponse)
def generate(req: GenerateRequest):
    try:
        payload = {
            "model": MODEL,
            "prompt": req.input,
            "stream": False,
        }

        r = requests.post(OLLAMA_URL, json=payload, timeout=60)
        r.raise_for_status()

        data = r.json()
        reply = data.get("response")

        if not reply:
            return GenerateResponse(
                success=False,
                reply=None,
                error="AI returned empty response",
                meta=None,
            )

        return GenerateResponse(
            success=True,
            reply=reply,
            error=None,
            meta={
                "model": MODEL,
                "tokens": data.get("eval_count", 0),
            },
        )

    except Exception as e:
        return GenerateResponse(
            success=False,
            reply=None,
            error=str(e),
            meta=None,
        )


# ----------------------------------------------------------------------
# ENTRYPOINT (CRUCIAAL)
# ----------------------------------------------------------------------

if __name__ == "__main__":
    print("[AI] Starting ELYSIA AI service on http://127.0.0.1:8123")
    uvicorn.run(
        "main:app",
        host="127.0.0.1",
        port=8123,
        log_level="info",
    )
