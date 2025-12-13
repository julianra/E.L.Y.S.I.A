# =========================================================
# 📍 FILE: elysia_ai/main.py
# 📝 ROLE:
#   Entrypoint voor de ELYSIA AI-service.
#   Start een lokale HTTP-server.
# =========================================================

import uvicorn
import yaml
from pathlib import Path
from api import app


BASE_DIR = Path(__file__).resolve().parent
CONFIG_PATH = BASE_DIR / "config.yaml"

with open(CONFIG_PATH, "r", encoding="utf-8") as f:
    CONFIG = yaml.safe_load(f)

HOST = CONFIG["service"]["host"]
PORT = int(CONFIG["service"]["port"])


if __name__ == "__main__":
    uvicorn.run(app, host=HOST, port=PORT)
