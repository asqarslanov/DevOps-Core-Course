# DevOps Info Service (Rust)

A high-performance web service that provides system information and health
status, implemented in Rust.

## Overview

This service exposes two RESTful endpoints that return JSON data about the
service, system environment, and health status. It's designed as a foundation
for DevOps monitoring tools and will evolve throughout the course.

## Prerequisites

- Rust 1.89 (install via [rustup](https://rustup.rs/))
- Cargo

## Running the Application

### Development Mode

```bash
cargo run
```

### Production Mode

```bash
cargo run --release
```

### With Custom Configuration

```bash
HOST=127.0.0.1 PORT=8080 DEBUG=true cargo run --release
```

## API Endpoints

### `GET /`

Returns comprehensive service and system information.

### `GET /health`

Simple health check endpoint for monitoring.

## Configuration

The service can be configured using environment variables:

| Variable | Default   | Description       |
| -------- | --------- | ----------------- |
| `HOST`   | `0.0.0.0` | Host to bind      |
| `PORT`   | `5000`    | Port to listen    |
| `DEBUG`  | `False`   | Enable debug mode |

## Testing

Test the endpoints using `curl`:

```bash
# Test main endpoint
curl http://localhost:5000/

# Test health endpoint
curl http://localhost:5000/health
```

## Dependencies

- `axum`: Web framework built on Tokio
- `tokio`: Async runtime
- `serde`: JSON serialization/deserialization
- `jiff`: Date and time handling
- `sysinfo`: System information collection
- `tracing`: Structured logging

## Performance (Release Build)

- **Memory usage**: ~4.1 MiB
- **Binary size**: ~3.2 MiB
