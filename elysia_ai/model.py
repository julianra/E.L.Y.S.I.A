# =========================================================
# 📍 FILE: elysia_ai/model.py
# 📝 ROLE:
#   Model-abstractie.
#   Praat met Ollama via HTTP.
# =========================================================

from __future__ import annotations

import requests
import yaml
from pathlib import Path
from typing import Tuple


# ---------------------------------------------------------
# Config laden (padvast)
# ---------------------------------------------------------

BASE_DIR = Path(__file__).resolve().parent
CONFIG_PATH = BASE_DIR / "config.yaml"

with open(CONFIG_PATH, "r", encoding="utf-8") as f:
    CONFIG = yaml.safe_load(f)

MODEL_NAME: str = CONFIG["model"]["name"]
OLLAMA_ENDPOINT: str = CONFIG["model"]["endpoint"].rstrip("/")
TIMEOUT_SECONDS: int = int(CONFIG["model"].get("timeout_seconds", 120))


# ---------------------------------------------------------
# Health helper
# ---------------------------------------------------------

def ollama_ready() -> bool:
    """
    Check if Ollama service is reachable.
    """
    try:
        r = requests.get(f"{OLLAMA_ENDPOINT}/api/tags", timeout=3)
        return r.ok
    except Exception:
        return False


# ---------------------------------------------------------
# Text generation
# ---------------------------------------------------------

def generate_text(prompt: str) -> Tuple[str, int]:
    payload = {
        "model": MODEL_NAME,
        "prompt": prompt,
        "stream": False,
        "options": {
            # 🔑 Cruciaal voor GTX 970
            "num_ctx": 2048,
            "num_predict": 128,
            "num_gpu": 0,        # force CPU
            "num_thread": 6,
            "temperature": 0.7
        }
    }

    try:
        response = requests.post(
            f"{OLLAMA_ENDPOINT}/api/generate",
            json=payload,
            timeout=TIMEOUT_SECONDS
        )
        response.raise_for_status()
    except requests.exceptions.RequestException as e:
        raise RuntimeError(f"Ollama request failed: {e}") from e

    data = response.json()
    output = (data.get("response") or "").strip()
    tokens = int(data.get("eval_count") or 0)

    return output, tokens
