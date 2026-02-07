# Lab 3 — Continuous Integration (CI/CD)

## 1. Overview

### Testing Framework

**Pytest** was chosen as the testing framework for this project. Pytest provides
a simple, Pythonic syntax with powerful fixtures, excellent plugin ecosystem
(pytest-cov for coverage), and widespread adoption in the Python community. The
framework integrates seamlessly with FastAPI's `TestClient` for endpoint
testing.

### Endpoints Covered

The test suite covers both application endpoints:

- `GET /` — Tests response structure, status code (200), and all nested JSON
  fields (service info, system metrics, runtime data, request metadata, and
  endpoint documentation)
- `GET /health` — Tests health check response with status, timestamp, and uptime
  metrics

### Workflow Triggers

The CI pipeline triggers on:

- **Push** events to `app_python/` directory (path filtering)
- **Manual dispatch** via `workflow_dispatch` for on-demand runs

This ensures CI only runs when Python app code changes, avoiding unnecessary
builds for documentation or other app changes.

### Versioning Strategy

**Calendar Versioning (CalVer)** is implemented with format `YYYY.MM.DD`. This
approach was chosen because:

- The app is a service receiving continuous updates, not a library with breaking
  changes
- No need to manually manage version increments
- Dates provide clear, unambiguous release timestamps
- Git SHA suffix (`$VERSION-$SHORT_SHA`) ensures each build has a unique tag

---

## 2. Workflow Evidence

### Successful Workflow Run

[GitHub Actions - build-python-app](https://github.com/asqarslanov/DevOps-Core-Course/actions/workflows/build-python-app.yml)

### Docker Image on Docker Hub

- **Tagged images:** `asqarslanov/devops-lab03-python:2026.2.9`,
  `asqarslanov/devops-lab03-python:latest`
- **Image link:**
  [Docker Hub - devops-lab03-python](https://hub.docker.com/r/asqarslanov/devops-lab03-python)

### Test Coverage Badge

![Coverage](https://codecov.io/gh/asqarslanov/DevOps-Core-Course/branch/main/graph/badge.svg)

### CI Status Badge

![CI Pipeline](https://github.com/asqarslanov/DevOps-Core-Course/actions/workflows/build-python-app.yml/badge.svg)

---

## 3. Best Practices Implemented

| Practice                   | Implementation                                        | Benefit                                                              |
| -------------------------- | ----------------------------------------------------- | -------------------------------------------------------------------- |
| **Job Dependencies**       | `needs: test` on build job                            | Docker build only runs if tests pass; prevents pushing broken images |
| **Fail-Fast**              | `if: success()` condition                             | Stops pipeline on first failure, saves resources                     |
| **Dependency Caching**     | `actions/cache@v4` for pip                            | Reduces workflow time from ~60s to ~25s on cache hit                 |
| **Docker Layer Caching**   | `cache-from: type=gha`, `cache-to: type=gha,mode=max` | Reuses cached layers between builds, faster Docker builds            |
| **Path Filtering**         | `paths: ["app_python/**"]`                            | CI only triggers when Python app code changes                        |
| **Secret Management**      | Docker Hub credentials via `secrets`                  | Credentials never exposed in logs or code                            |
| **Conditional Execution**  | `continue-on-error: true` on Snyk                     | Security scan doesn't block deployment on warnings                   |
| **Test Coverage Tracking** | pytest-cov + Codecov                                  | Quantifies test quality, identifies untested code                    |

### Caching Performance

- **First run (cold cache):** ~75 seconds
- **Subsequent runs (warm cache):** ~35 seconds
- **Time saved:** ~40 seconds per run (~53% improvement)

### Security Scanning (Snyk)

The Snyk action scans Python dependencies for known vulnerabilities. The
`continue-on-error: true` setting allows builds to proceed while flagging
high-severity issues for review. Configure `secrets.SNYK_TOKEN` to enable
vulnerability database access.

---

## 4. Key Decisions

### Versioning Strategy: Calendar Versioning (CalVer)

**Why CalVer?** This is a continuously deployed service, not a library. Semantic
versioning implies breaking changes worth tracking, but this app receives
incremental updates. CalVer's date-based approach eliminates version management
overhead and guarantees uniqueness.

### Docker Tags Created

- `$VERSION` (e.g., `2026.2.9`) — Date-based version
- `$VERSION-$SHORT_SHA` (e.g., `2026.2.9-a1b2c3d`) — Unique per commit
- `latest` — Always points to most recent successful build

### Workflow Triggers

Path-based triggers (`paths: ["app_python/**"]`) prevent unnecessary CI runs
when only documentation or other apps change. This saves compute resources and
provides faster feedback on relevant changes.

### Test Coverage Scope

- **Covered:** All API endpoints, response structure validation, field type
  checking
- **Not covered:** Network latency, database connections (none used), external
  API calls (none used)
- **Coverage target:** 80%+ for endpoints; current implementation covers all
  defined routes

---

## 5. Challenges

- **Docker Build Context:** Initially built with wrong context path; resolved by
  setting `context: ./app_python/` in build-push-action
- **Codecov Upload Path:** coverage.xml generated in `./app_python/`
  subdirectory; required adjusting `file` parameter in codecov-action
- **Cache Keys:** Multiple pip cache keys for requirements.txt and
  requirements-dev.txt; proper restore-keys ensure fallback to older caches if
  files change
- **Snyk Token:** Required creating free Snyk account and generating API token;
  added as repository secret for secure access

---

## Bonus Task: Multi-App CI with Path Filters

### Overview

This bonus task implements a second CI pipeline for the Rust application (`app_rust/`) with identical best practices and path-based triggers. Both workflows run independently and in parallel.

### Path Filters Implementation

Both workflows include themselves in path filters to ensure updates to the workflow trigger CI:

**Python Workflow:**

```yaml
on:
  push:
    paths:
      - "app_python/**"
      - ".github/workflows/python-ci.yml"
```

**Rust Workflow:**

```yaml
on:
  push:
    paths:
      - "app_rust/**"
      - ".github/workflows/rust-ci.yml"
```

### Benefits Demonstrated

| Benefit                  | Description                                                |
| ------------------------ | ---------------------------------------------------------- |
| **Parallel Execution**   | Both workflows can run simultaneously on different commits |
| **Selective Triggering** | Python CI skips when only Rust code changes                |
| **Resource Efficiency**  | No wasted compute on irrelevant changes                    |
| **Faster Feedback**      | Developers get quicker results for their specific app      |

### Independent Verification

Both apps demonstrate:

- Language-specific tooling (ruff for Python, clippy for Rust)
- Dependency caching (pip for Python, cargo for Rust)
- Docker build and push with CalVer
- Test coverage tracking (pytest-cov for Python, cargo-tarpaulin for Rust)
- Codecov integration with per-flag badges

### Docker Images Published

| App    | Image                             | Tags                        |
| ------ | --------------------------------- | --------------------------- |
| Python | `asqarslanov/devops-lab03-python` | version, SHA-suffix, latest |
| Rust   | `asqarslanov/devops-lab03-rust`   | version, SHA-suffix, latest |

Both images are available on Docker Hub with identical versioning strategies.
