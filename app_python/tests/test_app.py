import httpx
from fastapi.testclient import TestClient

from app import app


def http_get(route: str) -> httpx.Response:
    with TestClient(app) as client:
        response = client.get(route)
        return response


def test_get_index() -> None:
    response = http_get("/")
    assert response.status_code == 200
