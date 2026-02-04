# DevOps Info Service

A web service that provides system information and health status.

## Prerequisites

- The application works with the latest stable release of Python (currently,
  v3.14). Older versions may work but are not officially supported.

## Installation

```bash
python -m venv ./venv/
source ./venv/bin/activate
pip install --requirement=./requirements.txt
```

## Running the Application

```bash
python ./app.py

# With custom configuration
PORT=8080 HOST=127.0.0.1 DEBUG=True python ./app.py
```

## Docker (Containerization)

This project can be containerized with Docker. The Dockerfile in this repository
builds the application into a small, secure image and runs the FastAPI server.

- DockerHub: [`asqarslanov/devops-lab02-python`](https://hub.docker.com/repository/docker/asqarslanov/devops-lab02-python/general)

## API Endpoints

### `GET /`

Returns comprehensive service and system information.

### `GET /health`

Simple health check endpoint.

## Configuration

| Variable | Default   | Description       |
| -------- | --------- | ----------------- |
| `HOST`   | `0.0.0.0` | Host to bind      |
| `PORT`   | `5000`    | Port to listen    |
| `DEBUG`  | `False`   | Enable debug mode |
