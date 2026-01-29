"""
DevOps Info Service
Main application module
"""

import datetime
import multiprocessing
import os
import platform
import socket
from contextlib import asynccontextmanager
from datetime import UTC

import uvicorn
from dotenv import load_dotenv
from fastapi import FastAPI, Request, status
from fastapi.responses import JSONResponse
from pydantic import BaseModel


class ServiceInfo(BaseModel):
    name: str
    version: str
    description: str
    framework: str


class SystemInfo(BaseModel):
    hostname: str
    platform: str
    platform_version: str
    architecture: str
    cpu_count: int
    python_version: str

    @staticmethod
    def get() -> "SystemInfo":
        return SystemInfo(
            hostname=socket.gethostname(),
            platform=platform.system(),
            platform_version=platform.release(),
            architecture=platform.machine(),
            cpu_count=multiprocessing.cpu_count(),
            python_version=platform.python_version(),
        )


class UptimeInfo(BaseModel):
    seconds: int
    human: str

    @staticmethod
    def get(start_time: datetime.datetime) -> "UptimeInfo":
        delta = datetime.datetime.now(UTC) - start_time
        seconds = int(delta.total_seconds())
        hours = seconds // 3600
        minutes = (seconds % 3600) // 60
        return UptimeInfo(seconds=seconds, human=f"{hours} hours, {minutes} minutes")


class RuntimeInfo(BaseModel):
    uptime_seconds: int
    uptime_human: str
    current_time: datetime.datetime
    timezone: str = "UTC"

    @staticmethod
    def get(start_time: datetime.datetime) -> "RuntimeInfo":
        current_time = datetime.datetime.now(UTC)
        uptime = UptimeInfo.get(start_time)

        return RuntimeInfo(
            uptime_seconds=uptime.seconds,
            uptime_human=uptime.human,
            current_time=current_time,
        )


class RequestInfo(BaseModel):
    client_ip: str | None
    user_agent: str | None
    method: str
    path: str

    @staticmethod
    def get(request: Request) -> "RequestInfo":
        client_ip = None
        if request.client:
            client_ip = request.client.host

        return RequestInfo(
            client_ip=client_ip,
            user_agent=request.headers.get("user-agent"),
            method=request.method,
            path=request.url.path,
        )


class EndpointInfo(BaseModel):
    path: str
    method: str
    description: str | None


class GetIndexResponse(BaseModel):
    service: ServiceInfo
    system: SystemInfo
    runtime: RuntimeInfo
    request: RequestInfo
    endpoints: list[EndpointInfo]


class GetHealthResponse(BaseModel):
    status: str
    timestamp: datetime.datetime
    uptime_seconds: int


@asynccontextmanager
async def lifespan(app: FastAPI):
    app.state.start_time = datetime.datetime.now(UTC)

    yield

    app.state.start_time = None


load_dotenv()

app = FastAPI(
    title="devops-info-service",
    version="1.0.0",
    description="DevOps Info Service - Provides system information and health status",
    debug=os.getenv("DEBUG", "False").lower() == "true",
    lifespan=lifespan,
)


@app.get("/")
async def get_index(request: Request) -> GetIndexResponse:
    """Service information"""
    return GetIndexResponse(
        service=ServiceInfo(
            name=request.app.title,
            version=request.app.version,
            description="DevOps course info service",
            framework="FastAPI",
        ),
        system=SystemInfo.get(),
        runtime=RuntimeInfo.get(request.app.state.start_time),
        request=RequestInfo.get(request),
        endpoints=[
            EndpointInfo(path="/", method="GET", description=get_index.__doc__),
            EndpointInfo(path="/health", method="GET", description=get_health.__doc__),
        ],
    )


@app.get("/health")
async def get_health(request: Request) -> GetHealthResponse:
    """Health check"""
    runtime_info = RuntimeInfo.get(request.app.state.start_time)

    return GetHealthResponse(
        status="healthy",
        timestamp=datetime.datetime.now(UTC),
        uptime_seconds=runtime_info.uptime_seconds,
    )


@app.exception_handler(status.HTTP_404_NOT_FOUND)
async def not_found_handler(_request: Request, _exc: Exception) -> JSONResponse:
    return JSONResponse(
        status_code=status.HTTP_404_NOT_FOUND,
        content={"error": "Not Found", "message": "Endpoint does not exist"},
    )


@app.exception_handler(status.HTTP_500_INTERNAL_SERVER_ERROR)
async def internal_error_handler(_request: Request, _exc: Exception) -> JSONResponse:
    return JSONResponse(
        status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
        content={
            "error": "Internal Server Error",
            "message": "An unexpected error occurred",
        },
    )


def main() -> None:
    debug = os.getenv("DEBUG", "False").lower() == "true"
    uvicorn.run(
        "app:app",
        host=os.getenv("HOST", "0.0.0.0"),
        port=int(os.getenv("PORT", "5000")),
        reload=debug,
        log_level="debug" if debug else "info",
        access_log=True,
    )


if __name__ == "__main__":
    main()
