FROM lukemathwalker/cargo-chef:0.1.73-rust-1.93-slim-trixie AS chef
WORKDIR /app/

FROM chef AS planner
COPY ./ ./
RUN cargo chef prepare --recipe-path=./recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json ./recipe.json
RUN cargo chef cook --release --recipe-path=./recipe.json
COPY ./ ./
RUN cargo install --path=./

FROM debian:trixie-slim AS runtime
WORKDIR /app/
COPY --from=builder /usr/local/cargo/bin/devops-info-service /usr/local/bin/
RUN useradd --create-home --shell=/bin/bash appuser
USER appuser
EXPOSE 5000
ENTRYPOINT ["/usr/local/bin/devops-info-service"]
