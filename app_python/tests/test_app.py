from fastapi.testclient import TestClient

from app import app


def test_get_index() -> None:
    with TestClient(app) as client:
        response = client.get("/")
        assert response.status_code == 200
