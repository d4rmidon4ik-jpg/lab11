import os
from fastapi import FastAPI

app = FastAPI(title="Python Service", version="1.0.0")

SERVICE_NAME = os.getenv("SERVICE_NAME", "python")
GO_SERVICE_URL = os.getenv("GO_SERVICE_URL", "http://localhost:8080")


@app.get("/ping")
async def ping() -> dict:
    return {"message": "pong", "service": SERVICE_NAME}


@app.get("/health")
async def health() -> dict:
    return {"status": "ok"}


@app.get("/info")
async def info() -> dict:
    """Показывает конфигурацию — полезно для проверки сети."""
    return {
        "service": SERVICE_NAME,
        "go_service_url": GO_SERVICE_URL,
    }
