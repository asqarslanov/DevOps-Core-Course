# Lab 3 — Bonus Task: Multi-App CI

## Bonus Task Overview

This document describes the Rust application CI pipeline that runs alongside the
Python app CI, demonstrating multi-app CI with path-based triggers.

## Implementation

### Path-Based Triggers

Both CI workflows use path filters to ensure they only run when relevant code
changes:

**Python Workflow (`.github/workflows/python-ci.yml`):**

```yaml
on:
  push:
    paths:
      - "app_python/**"
      - ".github/workflows/python-ci.yml"
```

**Rust Workflow (`.github/workflows/rust-ci.yml`):**

```yaml
on:
  push:
    paths:
      - "app_rust/**"
      - ".github/workflows/rust-ci.yml"
```

### Benefits of Path Filters

1. **Reduced CI Runtime:** Workflows only run when their specific app code
   changes
2. **Resource Efficiency:** Avoids unnecessary builds for unchanged applications
3. **Parallel Execution:** Both workflows can run independently in parallel
4. **Faster Feedback:** Developers get quicker results for their specific
   changes

### Example Scenarios

| Change                 | Python CI  | Rust CI    |
| ---------------------- | ---------- | ---------- |
| `app_python/app.py`    | ✅ Runs    | ❌ Skipped |
| `app_rust/src/main.rs` | ❌ Skipped | ✅ Runs    |
| `README.md`            | ❌ Skipped | ❌ Skipped |
| Both apps changed      | ✅ Runs    | ✅ Runs    |

---

## Rust CI Workflow Details

### Language-Specific Tools

| Tool            | Purpose                          |
| --------------- | -------------------------------- |
| `cargo fmt`     | Code formatting check            |
| `cargo clippy`  | Linting (Rust's official linter) |
| `cargo nextest` | Fast test runner                 |

### Why Cargo Nextest?

**Cargo Nextest** is a next-generation test runner for Rust, offering:

- **Faster execution:** Up to 2-3x faster than standard `cargo test`
- **Better output:** Clearer failure messages and progress indicators
- **Parallel execution:** Optimized test scheduling
- **CI integration:** Official GitHub Action (`taiki-e/install-action@nextest`)

Nextest is installed in CI via the `taiki-e/install-action@nextest` action and
runs with `cargo nextest run`.

---

## Versioning Strategy

The Rust app uses **Calendar Versioning (CalVer)** consistent with the Python
app:

- Format: `YYYY.MM.DD`
- Tags: `$VERSION`, `$VERSION-$SHORT_SHA`, `latest`

---

## Workflow Evidence

### Successful Rust Workflow Run

[GitHub Actions - build-rust-app](https://github.com/asqarslanov/DevOps-Core-Course/actions/workflows/rust-ci.yml)

### Docker Image on Docker Hub

- **Image:** `asqarslanov/devops-lab03-rust`
- **Tags:** Version-based tags with SHA suffix

---

## Key Decisions

### Why Path Filters for Monorepo?

With both Python and Rust apps in the same repository, path filters prevent
cross-contamination of CI runs. A change to Python code shouldn't trigger Rust
builds, saving ~2-5 minutes per irrelevant run.

### Why Cargo Nextest?

Nextest provides significantly faster test execution compared to `cargo test`,
which is especially valuable in CI environments. It provides:

- Better parallelization
- Improved failure diagnostics
- Lower resource usage

### Testing Strategy

- **Unit tests:** Library-level tests in `src/lib.rs`
- **Test runner:** Cargo Nextest for performance
- **Coverage:** Not implemented (tarpaulin compatibility issues)
