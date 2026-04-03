from fastapi.testclient import TestClient
from main import app

client = TestClient(app)


def test_ping():
    resp = client.get("/ping")
    assert resp.status_code == 200
    data = resp.json()
    assert data["message"] == "pong"
    assert data["service"] == "python"


def test_health():
    resp = client.get("/health")
    assert resp.status_code == 200
    assert resp.json()["status"] == "ok"


def test_info():
    resp = client.get("/info")
    assert resp.status_code == 200
    data = resp.json()
    assert "service" in data
    assert "go_service_url" in data
    assert isinstance(data["go_service_url"], str)