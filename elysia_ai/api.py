# =========================================================
# 📍 FILE: elysia_ai/api.py
# 📝 ROLE:
#   HTTP API voor de AI-service.
#   Bevat GEEN businesslogica.
# =========================================================

from fastapi import FastAPI
from schemas import GenerateRequest, GenerateResponse
from model import generate_text, ollama_ready, MODEL_NAME

app = FastAPI()


@app.get("/health")
def health():
    return {
        "status": "ok",
        "ready": True,
        "model": MODEL_NAME,
        "ollama_ready": ollama_ready()
    }


@app.post("/generate", response_model=GenerateResponse)
def generate(req: GenerateRequest):
    try:
        output, tokens = generate_text(req.input)
        return GenerateResponse(
            success=True,
            output=output,
            tokens=tokens,
            error=None
        )
    except Exception as e:
        # Geen stacktrace lekken; enkel een gecontroleerde error string
        return GenerateResponse(
            success=False,
            output="",
            tokens=0,
            error=str(e)
        )
