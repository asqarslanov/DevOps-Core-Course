# DevOps Info Service

![CI Pipeline](https://github.com/asqarslanov/DevOps-Core-Course/actions/workflows/build-python-app.yml/badge.svg)
![Coverage](https://codecov.io/gh/asqarslanov/DevOps-Core-Course/branch/main/graph/badge.svg)

A web service that provides system information and health status.

## Prerequisites

- The application works with the latest stable release of Python (currently,
  v3.14). Older versions may work but are not officially supported.

## Installation

```sh
python -m venv ./venv/
source ./venv/bin/activate
pip install --requirement=./requirements.txt
```

## Running the Application

```sh
python ./app.py

# With custom configuration
PORT=8080 HOST=127.0.0.1 DEBUG=True python ./app.py
```

## Docker (Containerization)

This project can be containerized with Docker. The Dockerfile in this repository
builds the application into a small, secure image and runs the FastAPI server.

- DockerHub:
  [`asqarslanov/devops-lab02-python`](https://hub.docker.com/repository/docker/asqarslanov/devops-lab02-python/general)

## Testing

This project uses **pytest** for unit testing with `pytest-cov` for coverage
tracking.

### Running Tests

```sh
# Install test dependencies
pip install -r requirements-dev.txt

# Run all tests with coverage
pytest --cov=./ --cov-report=term --cov-report=xml

# Run tests only
pytest
```

### Test Coverage

Coverage reports are generated automatically and uploaded to Codecov on each CI
run. View coverage trends at
[codecov.io](https://codecov.io/gh/asqarslanov/DevOps-Core-Course).

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
