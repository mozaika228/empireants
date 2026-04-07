FROM rust:1.78-bookworm AS builder
WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY ui ./ui

RUN cargo build --release --bin observability_server --bin ui_server --bin scale_benchmark --bin scientific_validation

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN useradd --uid 10001 --create-home empireants

COPY --from=builder /app/target/release/observability_server /usr/local/bin/observability_server
COPY --from=builder /app/target/release/ui_server /usr/local/bin/ui_server
COPY --from=builder /app/target/release/scale_benchmark /usr/local/bin/scale_benchmark
COPY --from=builder /app/target/release/scientific_validation /usr/local/bin/scientific_validation
COPY --from=builder /app/ui /app/ui

RUN chown -R empireants:empireants /app
USER empireants

CMD ["observability_server", "0.0.0.0:9109", "100000", "384", "384"]
