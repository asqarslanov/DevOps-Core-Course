# DevOps Info Service (Rust)

![CI Pipeline](https://github.com/asqarslanov/DevOps-Core-Course/actions/workflows/rust-ci.yml/badge.svg)

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

This project uses Rust's built-in test framework with `cargo test` and **Cargo
Nextest** for faster test execution in CI.

### Running Tests

```bash
# Standard cargo test
cargo test

# Faster tests with Nextest (recommended for CI)
cargo nextest run
```

### CI Testing

The CI pipeline uses **Cargo Nextest** via the `taiki-e/install-action@nextest`
action for improved performance:

- Parallel test execution
- Better failure output
- Faster feedback loops

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
