# =========================================================
# 📍 FILE: elysia_ai/schemas.py
# 📝 ROLE:
#   Input- en outputschemas voor AI-requests.
#   Zorgt voor voorspelbaarheid en veiligheid.
# =========================================================

from pydantic import BaseModel, Field
from typing import Dict


class GenerateRequest(BaseModel):
    input: str
    context: Dict[str, str] = Field(default_factory=dict)


class GenerateResponse(BaseModel):
    success: bool
    output: str
    tokens: int
    error: str | None = None
