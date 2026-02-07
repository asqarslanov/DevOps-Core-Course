import httpx
from app import app
from fastapi.testclient import TestClient


def http_get(route: str) -> httpx.Response:
    with TestClient(app) as client:
        response = client.get(route)
        return response


def test_get_index() -> None:
    response = http_get("/")
    assert response.status_code == 200

    body = response.json()

    service = body["service"]
    assert type(service["name"]) is str
    assert type(service["version"]) is str
    assert type(service["description"]) is str
    assert type(service["framework"]) is str

    system = body["system"]
    assert type(system["hostname"]) is str
    assert type(system["platform"]) is str
    assert type(system["platform_version"]) is str
    assert type(system["architecture"]) is str
    assert type(system["cpu_count"]) is int
    assert type(system["python_version"]) is str

    runtime = body["runtime"]
    assert type(runtime["uptime_seconds"]) is int
    assert type(runtime["uptime_human"]) is str
    assert type(runtime["current_time"]) is str
    assert type(runtime["timezone"]) is str

    request = body["request"]
    if "client_ip" in request:
        assert type(request["client_ip"]) is str
    if "user_agent" in request:
        assert type(request["user_agent"]) is str
    assert type(request["method"]) is str
    assert type(request["path"]) is str

    endpoints = body["endpoints"]
    for endpoint in endpoints:
        assert type(endpoint["path"]) is str
        assert type(endpoint["method"]) is str
        if "description" in endpoint:
            assert type(endpoint["description"]) is str


def test_get_health() -> None:
    response = http_get("/health")
    assert response.status_code == 200

    body = response.json()

    assert type(body["status"]) is str
    assert type(body["timestamp"]) is str
    assert type(body["uptime_seconds"]) is int
